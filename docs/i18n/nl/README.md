[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit is een opdrachtregelprogramma dat ontwikkelaars helpt gestructureerde en consistente commitberichten te maken volgens de specificatie [Conventional Commits](https://www.conventionalcommits.org/). Het biedt een interactieve interface die gebruikers begeleidt bij het opstellen van een commitbericht met type, optionele scope, onderwerp, beschrijving en voettekst.

## Functies

- **Kernfunctionaliteit**
  - Interactieve selectie van committypes en scopes
  - Aanpasbare configuratie via `.commitkit.toml`
  - Handhaving van de maximale onderwerplengte
  - Ondersteuning voor meerregelige commitbeschrijvingen
  - Automatische invoervalidatie
  - Optionele `Signed-off-by`-regel bij commits
  - Platformonafhankelijke compatibiliteit

- **Uitgebreide functies**
  - Emoji-ondersteuning in commitberichten (✨, 🐛, enz.)
  - Commitsjablonen met plaatsaanduidingvervanging
  - Automatische generatie en bijwerking van het changelog
  - Git-hooks voor validatie en opmaak van commitberichten
  - Commitstatistieken en -analyse
  - Interactieve prompts voor sjabloonwaarden

## Vereisten

- **Rust**: versie 1.88.0 of hoger (MSRV)
- **Cargo**: inbegrepen bij de Rust-installatie

## Installatie

### Vanuit broncode

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Gebruik

Ga naar uw Git-repository en voer uit:

```shell
commitkit
```

### Opdrachtregelopties

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Sla git commit over en druk het bericht af naar stdout
    -v, --verbose           Uitgebreide uitvoer tonen
    -c, --config            Pad naar configuratiebestand (standaard: .commitkit.toml in huidige of persoonlijke map)
        --emoji             Emoji in commitberichten in-/uitschakelen
        --template          Een specifieke commitsjabloon gebruiken
        --changelog         Changelog bijwerken na het committen
        --install-hooks     Git-hooks installeren
        --stats             Commitstatistieken tonen
        --days              Aantal te analyseren dagen voor statistieken (standaard: alle)
        --validate          Een commitberichtbestand valideren
        --prepare-msg       Commitbericht voorbereiden (gebruikt door git-hook)
        --lang <LANG>       Weergavetaal (bijv. en, de, ja, zh-CN, pt-BR)
    -h, --help              Helpinformatie tonen
    -V, --version           Versie-informatie tonen
```

### Ondersteunde talen

CommitKit detecteert de weergavetaal via `LC_ALL`, `LC_MESSAGES` en `LANG`, of accepteert `--lang <LANG>`. Er worden 45 talen ondersteund, waaronder 6 rechts-naar-links-talen (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` en `ur`). Zie het volledige overzicht in de [Engelse README](../../../README.md).

### Interactief proces

Het hulpprogramma begeleidt u bij het maken van een conventioneel commitbericht:

1. Selecteer een committype (`feat`, `fix`, `docs`, enz.)
2. Selecteer een scope (optioneel)
3. Voer het commitonderwerp in
4. Voer de commitbeschrijving in (optioneel)
5. Voer de commitvoettekst in (optioneel)

Na het voltooien van deze stappen voert CommitKit `git commit` uit met het samengestelde bericht.

### Voorbeelden

```shell
# Basisgebruik
commitkit

# Proefmodus - toon het resulterende commitbericht zonder te committen
commitkit --dry-run

# Een aangepast configuratiebestand gebruiken
commitkit --config /path/to/custom-config.toml

# Uitgebreide uitvoer
commitkit --verbose

# Emoji's gebruiken in commitberichten
commitkit --emoji

# Een sjabloon gebruiken voor uw commit
commitkit --template feature

# CHANGELOG.md automatisch bijwerken
commitkit --changelog

# Git-hooks installeren
commitkit --install-hooks

# Commitstatistieken van de afgelopen 30 dagen tonen
commitkit --stats --days 30

# Een commitbericht valideren
commitkit --validate .git/COMMIT_EDITMSG
```

## Ontwikkeling

### Bouwen

```bash
# Debug-build
cargo build

# Release-build
cargo build --release
```

### Testen

```bash
# Alle tests uitvoeren
cargo test

# Tests met uitvoer uitvoeren
cargo test -- --nocapture
```

### Lint

```bash
# Controleren op lint-waarschuwingen
cargo clippy --all-targets --all-features

# Automatische oplossingen toepassen
cargo clippy --fix --allow-dirty
```

### Opmaak

```bash
# Opmaak controleren
cargo fmt -- --check

# Opmaak toepassen
cargo fmt
```

## Configuratie

CommitKit zoekt naar een configuratiebestand met de naam `.commitkit.toml` op de volgende locaties (in volgorde):

1. Huidige map
2. Persoonlijke map van de gebruiker

Als er geen configuratiebestand wordt gevonden, wordt er een standaardbestand aangemaakt in uw persoonlijke map.

Voorbeeldconfiguratie:

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

## Licentie

Dit project valt onder de MIT-licentie — zie het bestand [LICENSE](../../../LICENSE) voor details.

## Bijdragen

Bijdragen zijn welkom! Open gerust issues of dien pull requests in.

1. Fork de repository
2. Maak uw feature-branch aan (`git checkout -b feature/amazing-feature`)
3. Commit uw wijzigingen met conventionele commits
4. Push de branch (`git push origin feature/amazing-feature`)
5. Open een Pull Request
