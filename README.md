# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](./Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit is a command-line tool that helps developers create well-structured and consistent commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) specification. It provides an interactive interface to guide users through the process of constructing a commit message with a type, optional scope, subject, body, and footer.

## Features

- **Core Functionality**
  - Interactive selection of commit types and scopes
  - Customizable configuration via `.commitkit.toml`
  - Enforcement of maximum subject length
  - Support for multi-line commit bodies
  - Automatic validation of input
  - Optional signed-off commits
  - Cross-platform compatibility

- **Enhanced Features**
  - Emoji support in commit messages (✨, 🐛, etc.)
  - Commit templates with placeholder substitution
  - Automatic changelog generation and maintenance
  - Git hooks for commit message validation and formatting
  - Commit statistics and analysis
  - Interactive prompts for template values

## Prerequisites

- **Rust**: Version 1.88.0 or higher (MSRV)
- **Cargo**: Included with Rust installation

## Installation

### From source

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Usage

Navigate to your git repository and run:

```shell
commitkit
```

### Command-line options

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Skip git commit and print the message to stdout
    -v, --verbose           Show verbose output
    -c, --config            Path to config file (default: .commitkit.toml in current or home directory)
        --emoji             Toggle emoji in commit messages
        --template          Use a specific commit template
        --changelog         Update changelog after committing
        --install-hooks     Install git hooks
        --stats             Show commit statistics
        --days              Number of days to analyze for stats (default: all)
        --validate          Validate a commit message file
        --prepare-msg       Prepare commit message (used by git hook)
        --lang <LANG>       Display language (e.g. en, de, ja, zh-CN, pt-BR)
    -h, --help              Print help information
    -V, --version           Print version information
```

### Supported Languages

CommitKit detects the display language from `LC_ALL`, `LC_MESSAGES`, and `LANG`, or accepts `--lang <LANG>`. It supports 48 languages (including 6 right-to-left locales: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, and `ur`):

| Code | Language | README | Code | Language | README | Code | Language | README |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `en` | English | [English](README.md) | `pl` | Polski | [README](docs/i18n/pl/README.md) | `so-SO` | Soomaali | [README](docs/i18n/so-SO/README.md) |
| `de` | Deutsch | [README](docs/i18n/de/README.md) | `tr` | Türkçe | [README](docs/i18n/tr/README.md) | `sw-KE` | Kiswahili | [README](docs/i18n/sw-KE/README.md) |
| `es` | Español | [README](docs/i18n/es/README.md) | `sv` | Svenska | [README](docs/i18n/sv/README.md) | `xh-ZA` | isiXhosa | [README](docs/i18n/xh-ZA/README.md) |
| `fr` | Français | [README](docs/i18n/fr/README.md) | `vi` | Tiếng Việt | [README](docs/i18n/vi/README.md) | `yo-NG` | Yorùbá | [README](docs/i18n/yo-NG/README.md) |
| `it` | Italiano | [README](docs/i18n/it/README.md) | `id` | Bahasa Indonesia | [README](docs/i18n/id/README.md) | `zu-ZA` | isiZulu | [README](docs/i18n/zu-ZA/README.md) |
| `pt-BR` | Português (Brasil) | [README](docs/i18n/pt-BR/README.md) | `af-ZA` | Afrikaans | [README](docs/i18n/af-ZA/README.md) | `bn` | বাংলা | [README](docs/i18n/bn/README.md) |
| `nl` | Nederlands | [README](docs/i18n/nl/README.md) | `am-ET` | አማርኛ | [README](docs/i18n/am-ET/README.md) | `fa` | فارسی | [README](docs/i18n/fa/README.md) |
| `ru` | Русский | [README](docs/i18n/ru/README.md) | `ar-EG` | العربية (مصر) | [README](docs/i18n/ar-EG/README.md) | `he` | עברית | [README](docs/i18n/he/README.md) |
| `uk` | Українська | [README](docs/i18n/uk/README.md) | `ar-MA` | العربية (المغرب) | [README](docs/i18n/ar-MA/README.md) | `mr` | मराठी | [README](docs/i18n/mr/README.md) |
| `ja` | 日本語 | [README](docs/i18n/ja/README.md) | `fr-MA` | Français (Maroc) | [README](docs/i18n/fr-MA/README.md) | `ta` | தமிழ் | [README](docs/i18n/ta/README.md) |
| `ko` | 한국어 | [README](docs/i18n/ko/README.md) | `fr-SN` | Français (Sénégal) | [README](docs/i18n/fr-SN/README.md) | `te` | తెలుగు | [README](docs/i18n/te/README.md) |
| `zh-CN` | 简体中文 | [README](docs/i18n/zh-CN/README.md) | `ha-NG` | Hausa | [README](docs/i18n/ha-NG/README.md) | `th` | ไทย | [README](docs/i18n/th/README.md) |
| `zh-TW` | 繁體中文 | [README](docs/i18n/zh-TW/README.md) | `ig-NG` | Igbo | [README](docs/i18n/ig-NG/README.md) | `ur` | اردو | [README](docs/i18n/ur/README.md) |
| `ar` | العربية | [README](docs/i18n/ar/README.md) | `mg-MG` | Malagasy | [README](docs/i18n/mg-MG/README.md) | `ms` | Bahasa Melayu | [README](docs/i18n/ms/README.md) |
| `hi` | हिन्दी | [README](docs/i18n/hi/README.md) | `rw-RW` | Kinyarwanda | [README](docs/i18n/rw-RW/README.md) | `fil` | Filipino | [README](docs/i18n/fil/README.md) |
| `bs` | Bosanski | [README](docs/i18n/bs/README.md) | `hr` | Hrvatski | [README](docs/i18n/hr/README.md) | `sr` | Српски | [README](docs/i18n/sr/README.md) |

### Interactive process

The tool will guide you through the process of creating a conventional commit message:

1. Select a commit type (feat, fix, docs, etc.)
2. Select a scope (optional)
3. Enter the commit subject
4. Enter the commit body (optional)
5. Enter the commit footer (optional)

After completing these steps, CommitKit will run `git commit` with your constructed message.

### Examples

```shell
# Basic usage
commitkit

# Dry run - display the resulting commit message without committing
commitkit --dry-run

# Use a custom config file
commitkit --config /path/to/custom-config.toml

# Verbose output
commitkit --verbose

# Use emojis in commit messages
commitkit --emoji

# Use a template for your commit
commitkit --template feature

# Automatically update CHANGELOG.md
commitkit --changelog

# Install git hooks
commitkit --install-hooks

# Show commit statistics for the last 30 days
commitkit --stats --days 30

# Validate a commit message
commitkit --validate .git/COMMIT_EDITMSG
```

## Development

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Test

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Lint

```bash
# Check for linting issues
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty
```

### Format

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Configuration

CommitKit looks for a configuration file named `.commitkit.toml` in the following locations (in order):

1. Current directory
2. User's home directory

If no configuration file is found, a default one will be generated in your home directory.

Example configuration:

```toml
# CommitKit Configuration

# Whether to add a Signed-off-by line to commits (-s flag)
sign_off_commits = false

# Commit message prefixes following Conventional Commits specification
prefixes = [
    { title = "feat", description = "A new feature" },
    { title = "fix", description = "A bug fix" },
    { title = "docs", description = "Documentation changes" },
    { title = "style", description = "Changes that do not affect code meaning" },
    { title = "refactor", description = "Code change that neither fixes a bug nor adds a feature" },
    { title = "perf", description = "Code change that improves performance" },
    { title = "test", description = "Adding missing tests or correcting existing tests" },
    { title = "build", description = "Changes that affect the build system or external dependencies" },
    { title = "ci", description = "Changes to CI configuration files and scripts" },
    { title = "chore", description = "Other changes that don't modify src or test files" },
    { title = "revert", description = "Reverts a previous commit" }
]

# Optional scopes to categorize changes
scopes = [
    "core",
    "ui",
    "docs",
    "tests",
    "deps"
]

# Subject line length constraints
max_subject_len = 72
min_subject_len = 3

# Feature toggles
use_emoji = true                      # Add emojis to commit messages
update_changelog = true               # Update CHANGELOG.md automatically
validate_commit_msg = true            # Validate commit messages against conventional format

# Templates directory for custom commit templates
templates_dir = "~/.commitkit/templates"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using conventional commits
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
