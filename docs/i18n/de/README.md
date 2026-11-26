[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit ist ein Kommandozeilenwerkzeug, das Entwickler dabei unterstützt, gut strukturierte und konsistente Commit-Nachrichten gemäß der Spezifikation [Conventional Commits](https://www.conventionalcommits.org/) zu erstellen. Es bietet eine interaktive Benutzeroberfläche, die Benutzer Schritt für Schritt durch das Verfassen einer Commit-Nachricht mit Typ, optionalem Bereich, Betreff, Textkörper und Fußzeile führt.

## Funktionen

- **Kernfunktionen**
  - Interaktive Auswahl von Commit-Typen und Bereichen
  - Anpassbare Konfiguration über `.commitkit.toml`
  - Durchsetzung der maximalen Betrefflänge
  - Unterstützung mehrzeiliger Commit-Textkörper
  - Automatische Validierung der Eingaben
  - Optionale `Signed-off-by`-Zeile in Commits
  - Plattformübergreifende Kompatibilität

- **Erweiterte Funktionen**
  - Emoji-Unterstützung in Commit-Nachrichten (✨, 🐛 usw.)
  - Commit-Vorlagen mit Platzhalterersetzung
  - Automatische Erstellung und Pflege des Changelogs
  - Git-Hooks zur Validierung und Formatierung von Commit-Nachrichten
  - Commit-Statistiken und Auswertungen
  - Interaktive Eingabeaufforderungen für Vorlagenwerte

## Voraussetzungen

- **Rust**: Version 1.88.0 oder höher (MSRV)
- **Cargo**: In der Rust-Installation enthalten

## Installation

### Aus dem Quellcode

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Verwendung

Wechseln Sie in Ihr Git-Repository und führen Sie folgenden Befehl aus:

```shell
commitkit
```

### Kommandozeilenoptionen

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit überspringen und Nachricht auf stdout ausgeben
    -v, --verbose           Ausführliche Ausgabe anzeigen
    -c, --config            Pfad zur Konfigurationsdatei (Standard: .commitkit.toml im aktuellen oder Home-Verzeichnis)
        --emoji             Emojis in Commit-Nachrichten umschalten
        --template          Bestimmte Commit-Vorlage verwenden
        --changelog         Changelog nach dem Commit aktualisieren
        --install-hooks     Git-Hooks installieren
        --stats             Commit-Statistiken anzeigen
        --days              Anzahl der zu analysierenden Tage für Statistiken (Standard: alle)
        --validate          Eine Commit-Nachrichtendatei validieren
        --prepare-msg       Commit-Nachricht vorbereiten (wird vom Git-Hook verwendet)
        --lang <LANG>       Anzeigesprache (z. B. en, de, ja, zh-CN, pt-BR)
    -h, --help              Hilfeinformationen anzeigen
    -V, --version           Versionsinformationen anzeigen
```

### Unterstützte Sprachen

CommitKit erkennt die Anzeigesprache automatisch über `LC_ALL`, `LC_MESSAGES` und `LANG` oder akzeptiert `--lang <LANG>`. Es werden 45 Sprachen unterstützt, darunter 6 Rechts-nach-links-Sprachen (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`). Die vollständige Sprachübersicht finden Sie in der [englischen README](../../../README.md).

### Interaktiver Ablauf

Das Werkzeug führt Sie durch die Erstellung einer Conventional-Commit-Nachricht:

1. Commit-Typ auswählen (`feat`, `fix`, `docs` usw.)
2. Bereich auswählen (optional)
3. Commit-Betreff eingeben
4. Commit-Textkörper eingeben (optional)
5. Commit-Fußzeile eingeben (optional)

Nach Abschluss dieser Schritte führt CommitKit `git commit` mit der erstellten Nachricht aus.

### Beispiele

```shell
# Grundlegende Verwendung
commitkit

# Testlauf - erstellte Commit-Nachricht ohne Commit anzeigen
commitkit --dry-run

# Benutzerdefinierte Konfigurationsdatei verwenden
commitkit --config /path/to/custom-config.toml

# Ausführliche Ausgabe
commitkit --verbose

# Emojis in Commit-Nachrichten verwenden
commitkit --emoji

# Vorlage für den Commit verwenden
commitkit --template feature

# CHANGELOG.md automatisch aktualisieren
commitkit --changelog

# Git-Hooks installieren
commitkit --install-hooks

# Commit-Statistiken der letzten 30 Tage anzeigen
commitkit --stats --days 30

# Eine Commit-Nachricht validieren
commitkit --validate .git/COMMIT_EDITMSG
```

## Entwicklung

### Erstellen

```bash
# Debug-Build
cargo build

# Release-Build
cargo build --release
```

### Testen

```bash
# Alle Tests ausführen
cargo test

# Tests mit Ausgabe ausführen
cargo test -- --nocapture
```

### Linting

```bash
# Auf Linting-Probleme prüfen
cargo clippy --all-targets --all-features

# Automatische Korrekturen anwenden
cargo clippy --fix --allow-dirty
```

### Formatierung

```bash
# Formatierung prüfen
cargo fmt -- --check

# Formatierung anwenden
cargo fmt
```

## Konfiguration

CommitKit sucht in der folgenden Reihenfolge nach einer Konfigurationsdatei namens `.commitkit.toml`:

1. Aktuelles Verzeichnis
2. Home-Verzeichnis des Benutzers

Wenn keine Konfigurationsdatei gefunden wird, wird automatisch eine Standardkonfiguration in Ihrem Home-Verzeichnis erstellt.

Beispielkonfiguration:

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

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz – Einzelheiten finden Sie in der Datei [LICENSE](../../../LICENSE).

## Mitwirken

Beiträge sind willkommen! Eröffnen Sie gerne Issues oder reichen Sie Pull Requests ein.

1. Forken Sie das Repository
2. Erstellen Sie Ihren Feature-Branch (`git checkout -b feature/amazing-feature`)
3. Committen Sie Ihre Änderungen im Conventional-Commits-Format
4. Pushen Sie den Branch (`git push origin feature/amazing-feature`)
5. Eröffnen Sie einen Pull Request
