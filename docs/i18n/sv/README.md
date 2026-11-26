[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit är ett kommandoradsverktyg som hjälper utvecklare att skapa välstrukturerade och konsekventa commit-meddelanden enligt specifikationen [Conventional Commits](https://www.conventionalcommits.org/). Verktyget har ett interaktivt gränssnitt som vägleder användaren steg för steg när ett meddelande med typ, valfritt omfång, rubrik, brödtext och sidfot skapas.

## Funktioner

- **Grundfunktioner**
  - Interaktivt val av commit-typer och omfång
  - Anpassningsbar konfiguration via `.commitkit.toml`
  - Kontroll av maximal rubriklängd
  - Stöd för flerradiga commit-brödtexter
  - Automatisk validering av inmatning
  - Valfri `Signed-off-by`-rad i commits
  - Plattformsoberoende kompatibilitet

- **Utökade funktioner**
  - Stöd för emoji i commit-meddelanden (✨, 🐛 m.fl.)
  - Commit-mallar med platshållarersättning
  - Automatisk generering och uppdatering av ändringslogg
  - Git-hooks för validering och formatering av commit-meddelanden
  - Commit-statistik och analys
  - Interaktiva frågor för mallvärden

## Förutsättningar

- **Rust**: Version 1.88.0 eller högre (MSRV)
- **Cargo**: Ingår i Rust-installationen

## Installation

### Från källkod

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Användning

Gå till ditt Git-arkiv och kör:

```shell
commitkit
```

### Kommandoradsalternativ

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Hoppa över git commit och skriv ut meddelandet till stdout
    -v, --verbose           Visa utförlig utdata
    -c, --config            Sökväg till konfigurationsfil (standard: .commitkit.toml i aktuell eller hemkatalog)
        --emoji             Växla emoji i commit-meddelanden
        --template          Använd en specifik commit-mall
        --changelog         Uppdatera ändringslogg efter commit
        --install-hooks     Installera git-hooks
        --stats             Visa commit-statistik
        --days              Antal dagar att analysera för statistik (standard: alla)
        --validate          Validera en fil med commit-meddelande
        --prepare-msg       Förbered commit-meddelande (används av git-hook)
        --lang <LANG>       Visningsspråk (t.ex. en, de, ja, zh-CN, pt-BR)
    -h, --help              Visa hjälpinformation
    -V, --version           Visa versionsinformation
```

### Språk som stöds

CommitKit identifierar visningsspråket via `LC_ALL`, `LC_MESSAGES` och `LANG`, eller tar emot `--lang <LANG>`. Verktyget stöder 45 språk, inklusive 6 höger-till-vänster-språk (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` och `ur`). Se den fullständiga tabellen i den [engelska README-filen](../../../README.md).

### Interaktiv process

Verktyget vägleder dig genom skapandet av ett konventionellt commit-meddelande:

1. Välj en commit-typ (`feat`, `fix`, `docs` osv.)
2. Välj ett omfång (valfritt)
3. Ange commit-rubrik
4. Ange brödtext för commit (valfritt)
5. Ange sidfot för commit (valfritt)

När stegen är klara kör CommitKit `git commit` med det skapade meddelandet.

### Exempel

```shell
# Grundläggande användning
commitkit

# Torrkörning - visa det färdiga meddelandet utan att göra en commit
commitkit --dry-run

# Använd en anpassad konfigurationsfil
commitkit --config /path/to/custom-config.toml

# Utförlig utdata
commitkit --verbose

# Använd emojis i commit-meddelanden
commitkit --emoji

# Använd en mall för din commit
commitkit --template feature

# Uppdatera CHANGELOG.md automatiskt
commitkit --changelog

# Installera git-hooks
commitkit --install-hooks

# Visa commit-statistik för de senaste 30 dagarna
commitkit --stats --days 30

# Validera ett commit-meddelande
commitkit --validate .git/COMMIT_EDITMSG
```

## Utveckling

### Bygg

```bash
# Felsökningsbygge
cargo build

# Utgivningsbygge
cargo build --release
```

### Test

```bash
# Kör alla tester
cargo test

# Kör tester med utdata
cargo test -- --nocapture
```

### Lint

```bash
# Kontrollera lint-varningar
cargo clippy --all-targets --all-features

# Tillämpa automatiska korrigeringar
cargo clippy --fix --allow-dirty
```

### Formatering

```bash
# Kontrollera formatering
cargo fmt -- --check

# Tillämpa formatering
cargo fmt
```

## Konfiguration

CommitKit letar efter en konfigurationsfil med namnet `.commitkit.toml` på följande platser (i ordning):

1. Aktuell katalog
2. Användarens hemkatalog

Om ingen konfigurationsfil hittas genereras en standardfil i din hemkatalog.

Exempelkonfiguration:

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

## Licens

Detta projekt är licensierat under MIT-licensen — se filen [LICENSE](../../../LICENSE) för detaljer.

## Bidra

Bidrag är varmt välkomna! Öppna gärna ärenden eller skicka pull requests.

1. Forka arkivet
2. Skapa din funktionsgren (`git checkout -b feature/amazing-feature`)
3. Gör commits med konventionella commit-meddelanden
4. Pusha grenen (`git push origin feature/amazing-feature`)
5. Öppna en Pull Request
