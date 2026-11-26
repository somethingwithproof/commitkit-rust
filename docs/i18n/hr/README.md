[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit je alat naredbenog retka koji razvojnim programerima pomaže u oblikovanju dobro strukturiranih i dosljednih poruka commita prema specifikaciji [Conventional Commits](https://www.conventionalcommits.org/). Pruža interaktivno sučelje koje korisnike vodi kroz sastavljanje poruke commita s vrstom, neobaveznim opsegom, naslovom, tijelom i podnožjem.

## Značajke

- **Osnovne mogućnosti**
  - Interaktivni odabir vrsta commita i opsega
  - Prilagodljive postavke putem datoteke `.commitkit.toml`
  - Nadzor najveće dopuštene duljine naslova
  - Podrška za višeredna tijela commita
  - Automatska provjera valjanosti unosa
  - Neobavezni redak `Signed-off-by` u commitovima
  - Višeplatformska kompatibilnost

- **Proširene mogućnosti**
  - Podrška za emojije u porukama commita (✨, 🐛 itd.)
  - Predlošci za commit sa zamjenom rezerviranih mjesta
  - Automatsko generiranje i održavanje zapisnika promjena
  - Git hookovi za provjeru valjanosti i oblikovanje poruka commita
  - Statistika i analiza commitova
  - Interaktivni upiti za vrijednosti predložaka

## Preduvjeti

- **Rust**: inačica 1.88.0 ili novija (MSRV)
- **Cargo**: uključen u instalaciju jezika Rust

## Instalacija

### Iz izvornog koda

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Upotreba

Otvorite svoj git repozitorij i pokrenite:

```shell
commitkit
```

### Opcije naredbenog retka

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

CommitKit prepoznaje jezik prikaza iz varijabli okruženja `LC_ALL`, `LC_MESSAGES` i `LANG`, ili prihvaća opciju `--lang <LANG>`. Podržava 48 jezika (uključujući 6 jezika pisanih zdesna nalijevo: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` i `ur`). Cjelovitu tablicu potražite u [engleskoj datoteci README](../../../README.md).

### Interaktivni postupak

Alat vas vodi kroz postupak izrade konvencionalne poruke commita:

1. Odaberite vrstu commita (feat, fix, docs itd.)
2. Odaberite opseg (neobavezno)
3. Unesite naslov commita
4. Unesite tijelo commita (neobavezno)
5. Unesite podnožje commita (neobavezno)

Nakon dovršetka ovih koraka, CommitKit pokreće `git commit` s vašom sastavljenom porukom.

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

### Oblikovanje

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Postavke

CommitKit traži datoteku s postavkama naziva `.commitkit.toml` na sljedećim lokacijama (redom):

1. Trenutačna mapa
2. Osobna mapa korisnika

Ako datoteka s postavkama nije pronađena, zadana će se datoteka izraditi u vašoj osobnoj mapi.

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

Ovaj je projekt licenciran pod MIT licencom - pojedinosti potražite u datoteci [LICENSE](../../../LICENSE).

## Doprinos projektu

Doprinosi su dobrodošli! Slobodno otvorite prijave pogrešaka ili pošaljite zahtjeve za spajanje (Pull Request).

1. Napravite fork repozitorija
2. Stvorite svoju granu za značajku (`git checkout -b feature/amazing-feature`)
3. Pohranite promjene koristeći konvencionalne poruke commita
4. Pošaljite granu na poslužitelj (`git push origin feature/amazing-feature`)
5. Otvorite Pull Request
