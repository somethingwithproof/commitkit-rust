[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

Ang CommitKit ay isang command-line tool na tumutulong sa mga developer na lumikha ng maayos at pare-parehong mga mensahe ng commit na sumusunod sa pamantayang [Conventional Commits](https://www.conventionalcommits.org/). Nagbibigay ito ng interactive na interface na gumagabay sa bawat hakbang sa pagbuo ng mensahe ng commit na may uri, opsyonal na saklaw, paksa, katawan, at footer.

## Mga Tampok

- **Pangunahing Kakayahan**
  - Interactive na pagpili ng mga uri ng commit at saklaw
  - Nako-customize na configuration gamit ang `.commitkit.toml`
  - Pagpapatupad ng maximum na haba ng paksa
  - Suporta para sa maraming linyang katawan ng commit
  - Awtomatikong pag-validate ng input
  - Opsyonal na linyang `Signed-off-by` sa mga commit
  - Cross-platform na compatibility

- **Mga Dagdag na Tampok**
  - Suporta sa emoji sa mga mensahe ng commit (✨, 🐛, at iba pa)
  - Mga template ng commit na may pagpapalit ng placeholder
  - Awtomatikong paggawa at pag-update ng changelog
  - Mga Git hook para sa pag-validate at pag-format ng mga mensahe ng commit
  - Mga estadistika at pagsusuri ng commit
  - Interactive na prompt para sa mga halaga ng template

## Mga Kinakailangan

- **Rust**: Bersyon 1.88.0 o mas bago (MSRV)
- **Cargo**: Kasama sa pag-install ng Rust

## Pag-install

### Mula sa source code

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Paggamit

Pumunta sa iyong Git repository at patakbuhin:

```shell
commitkit
```

### Mga opsyon sa command line

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Laktawan ang git commit at i-print ang mensahe sa stdout
    -v, --verbose           Ipakita ang detalyadong output
    -c, --config            Landas patungo sa config file (default: .commitkit.toml sa kasalukuyan o home directory)
        --emoji             I-on o i-off ang emoji sa mga mensahe ng commit
        --template          Gumamit ng partikular na template ng commit
        --changelog         I-update ang changelog pagkatapos mag-commit
        --install-hooks     I-install ang mga git hook
        --stats             Ipakita ang mga estadistika ng commit
        --days              Bilang ng araw na susuriin para sa mga estadistika (default: lahat)
        --validate          Suriin ang bisa ng isang file ng mensahe ng commit
        --prepare-msg       Ihanda ang mensahe ng commit (ginagamit ng git hook)
        --lang <LANG>       Wika ng display (hal. en, fil, ms, id, ja)
    -h, --help              I-print ang impormasyon ng tulong
    -V, --version           I-print ang impormasyon ng bersyon
```

### Mga Sinusuportahang Wika

Awtomatikong kinikilala ng CommitKit ang wika ng display mula sa `LC_ALL`, `LC_MESSAGES`, at `LANG`, o tumatanggap ng `--lang <LANG>`. Sinusuportahan nito ang 45 wika, kabilang ang 6 na wikang kanan-pakaliwa (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, at `ur`). Tingnan ang kumpletong talahanayan sa [README sa Ingles](../../../README.md).

### Interactive na proseso

Gagabayan ka ng tool sa paggawa ng isang conventional commit na mensahe:

1. Pumili ng uri ng commit (`feat`, `fix`, `docs`, atbp.)
2. Pumili ng saklaw (opsyonal)
3. Ilagay ang paksa ng commit
4. Ilagay ang katawan ng commit (opsyonal)
5. Ilagay ang footer ng commit (opsyonal)

Matapos ang mga hakbang na ito, isasagawa ng CommitKit ang `git commit` gamit ang nabuong mensahe.

### Mga Halimbawa

```shell
# Pangunahing paggamit
commitkit

# Dry run - ipakita ang nabuong mensahe nang hindi nagko-commit
commitkit --dry-run

# Gumamit ng custom na config file
commitkit --config /path/to/custom-config.toml

# Detalyadong output
commitkit --verbose

# Gumamit ng emoji sa mga mensahe ng commit
commitkit --emoji

# Gumamit ng template para sa iyong commit
commitkit --template feature

# Awtomatikong i-update ang CHANGELOG.md
commitkit --changelog

# I-install ang mga git hook
commitkit --install-hooks

# Ipakita ang mga estadistika ng commit sa nakalipas na 30 araw
commitkit --stats --days 30

# I-validate ang isang mensahe ng commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Pagpapaunlad

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Pagsusuri (Test)

```bash
# Patakbuhin ang lahat ng pagsusuri
cargo test

# Patakbuhin ang mga pagsusuri nang may output
cargo test -- --nocapture
```

### Lint

```bash
# Suriin ang mga babala ng Clippy
cargo clippy --all-targets --all-features

# Ilapat ang awtomatikong pag-aayos
cargo clippy --fix --allow-dirty
```

### Pag-format

```bash
# Suriin ang pag-format
cargo fmt -- --check

# Ilapat ang pag-format
cargo fmt
```

## Configuration

Hinahanap ng CommitKit ang configuration file na `.commitkit.toml` sa mga sumusunod na lokasyon (ayon sa pagkakasunod):

1. Kasalukuyang directory
2. Home directory ng user

Kung walang makitang configuration file, awtomatikong gagawa ng default na file sa iyong home directory.

Halimbawa ng configuration:

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

## Lisensya

Ang proyektong ito ay lisensyado sa ilalim ng MIT License — tingnan ang [LICENSE](../../../LICENSE) file para sa mga detalye.

## Pag-aambag

Malugod na tinatanggap ang mga ambag! Huwag mag-atubiling magbukas ng mga issue o magsumite ng pull request.

1. I-fork ang repository
2. Gumawa ng iyong feature branch (`git checkout -b feature/amazing-feature`)
3. I-commit ang iyong mga pagbabago gamit ang conventional commits
4. I-push ang branch (`git push origin feature/amazing-feature`)
5. Magbukas ng Pull Request
