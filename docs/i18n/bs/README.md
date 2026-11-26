[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit je alat za komandnu liniju koji pomaže programerima da pišu dobro strukturirane i dosljedne commit poruke prema specifikaciji [Conventional Commits](https://www.conventionalcommits.org/). Pruža interaktivni interfejs koji vodi korisnike kroz sastavljanje commit poruke s tipom, opcionalnim opsegom, naslovom, tijelom i podnožjem.

## Mogućnosti

- **Osnovne funkcionalnosti**
  - Interaktivni odabir tipova commita i opsega
  - Prilagodljive postavke putem datoteke `.commitkit.toml`
  - Kontrola maksimalne dužine naslova
  - Podrška za višeredna tijela commita
  - Automatska provjera unosa
  - Opcionalna `Signed-off-by` linija u commitovima
  - Višeplatformska kompatibilnost

- **Napredne mogućnosti**
  - Podrška za emoji u commit porukama (✨, 🐛 itd.)
  - Šabloni za commit sa zamjenom parametara
  - Automatsko generisanje i održavanje dnevnika promjena
  - Git hookovi za provjeru i formatiranje commit poruka
  - Statistika i analiza commitova
  - Interaktivni upiti za vrijednosti u šablonima

## Preduslovi

- **Rust**: verzija 1.88.0 ili novija (MSRV)
- **Cargo**: uključen uz instalaciju Rusta

## Instalacija

### Iz izvornog koda

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Korištenje

Otvorite svoj git repozitorij i pokrenite:

```shell
commitkit
```

### Opcije komandne linije

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

### Podržani jezici

CommitKit prepoznaje jezik prikaza iz varijabli okruženja `LC_ALL`, `LC_MESSAGES` i `LANG`, ili prihvata opciju `--lang <LANG>`. Podržava 48 jezika (uključujući 6 jezika pisanih zdesna nalijevo: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` i `ur`). Potpunu tabelu pogledajte u [engleskom README fajlu](../../../README.md).

### Interaktivni postupak

Alat vas vodi kroz kreiranje konvencionalne commit poruke:

1. Odaberite tip commita (feat, fix, docs itd.)
2. Odaberite opseg (opcionalno)
3. Unesite naslov commita
4. Unesite tijelo commita (opcionalno)
5. Unesite podnožje commita (opcionalno)

Nakon završetka ovih koraka, CommitKit pokreće `git commit` s vašom sastavljenom porukom.

### Primjeri

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

## Razvoj

### Izgradnja

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testiranje

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Provjera koda (Lint)

```bash
# Check for linting issues
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty
```

### Formatiranje

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Postavke

CommitKit traži konfiguracijsku datoteku pod nazivom `.commitkit.toml` na sljedećim lokacijama (po redoslijedu):

1. Trenutni direktorij
2. Korisnički početni direktorij

Ako konfiguracijska datoteka nije pronađena, zadana datoteka će biti generisana u vašem početnom direktoriju.

Primjer postavki:

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

## Licenca

Ovaj projekat je licenciran pod MIT licencom - za detalje pogledajte datoteku [LICENSE](../../../LICENSE).

## Doprinosi

Doprinosi su dobrodošli! Slobodno otvorite prijave problema ili pošaljite zahtjeve za spajanje (Pull Request).

1. Napravite fork repozitorija
2. Kreirajte svoju granu za funkcionalnost (`git checkout -b feature/amazing-feature`)
3. Sačuvajte izmjene pomoću konvencionalnih commit poruka
4. Pošaljite granu na server (`git push origin feature/amazing-feature`)
5. Otvorite Pull Request
