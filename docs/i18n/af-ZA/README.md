[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit is 'n bevelreël-hulpmiddel wat ontwikkelaars help om goed gestruktureerde en konsekwente commit-boodskappe volgens die [Conventional Commits](https://www.conventionalcommits.org/)-spesifikasie te skep. Dit bied 'n interaktiewe koppelvlak wat gebruikers stap vir stap lei om 'n commit-boodskap met 'n tipe, opsionele bestek, onderwerp, liggaam en voetskrif saam te stel.

## Kenmerke

- **Kernfunksionaliteit**
  - Interaktiewe keuse van commit-tipes en bestekke
  - Aanpasbare konfigurasie via `.commitkit.toml`
  - Handhawing van maksimum onderwerplengte
  - Ondersteuning vir meerreëlige commit-liggame
  - Outomatiese validering van invoer
  - Opsionele `Signed-off-by`-reël in commits
  - Kruisplatform-versoenbaarheid

- **Uitgebreide kenmerke**
  - Emoji-ondersteuning in commit-boodskappe (✨, 🐛, ens.)
  - Commit-sjablone met plekhouervervanging
  - Outomatiese generering en bywerking van veranderingslogboeke
  - Git-hooks vir validering en formatering van commit-boodskappe
  - Commit-statistieke en ontleding
  - Interaktiewe aanwysings vir sjabloonwaardes

## Voorvereistes

- **Rust**: Weergawe 1.88.0 of hoër (MSRV)
- **Cargo**: Ingesluit by die Rust-installasie

## Installasie

### Vanaf bronkode

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Gebruik

Navigeer na jou Git-bewaarplek en voer uit:

```shell
commitkit
```

### Bevelreëlopsies

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Slaan git commit oor en druk die boodskap na stdout
    -v, --verbose           Wys uitgebreide afvoer
    -c, --config            Pad na konfigurasielêer (verstek: .commitkit.toml in die huidige of tuisgids)
        --emoji             Skakel emoji in commit-boodskappe aan of af
        --template          Gebruik 'n spesifieke commit-sjabloon
        --changelog         Dateer die veranderingslogboek op na die commit
        --install-hooks     Installeer git-hooks
        --stats             Wys commit-statistieke
        --days              Aantal dae om vir statistieke te ontleed (verstek: alle)
        --validate          Valideer 'n commit-boodskaplêer
        --prepare-msg       Berei commit-boodskap voor (gebruik deur git-hook)
        --lang <LANG>       Vertoontaal (bv. en, af-ZA, sw-KE, ja, ar)
    -h, --help              Druk hulpinligting
    -V, --version           Druk weergawe-inligting
```

### Ondersteunde tale

CommitKit bespeur die vertoontaal outomaties vanaf `LC_ALL`, `LC_MESSAGES` en `LANG`, of aanvaar `--lang <LANG>`. Dit ondersteun 45 tale, insluitend 6 regs-na-links-tale (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` en `ur`). Sien die volledige tabel in die [Engelse README](../../../README.md).

### Interaktiewe proses

Die hulpmiddel lei jou deur die skep van 'n konvensionele commit-boodskap:

1. Kies 'n commit-tipe (`feat`, `fix`, `docs`, ens.)
2. Kies 'n bestek (opsioneel)
3. Voer die commit-onderwerp in
4. Voer die commit-liggaam in (opsioneel)
5. Voer die commit-voetskrif in (opsioneel)

Nadat jy hierdie stappe voltooi het, voer CommitKit `git commit` met die saamgestelde boodskap uit.

### Voorbeelde

```shell
# Basiese gebruik
commitkit

# Toetslopie - wys die gevolglike commit-boodskap sonder om te commit
commitkit --dry-run

# Gebruik 'n pasgemaakte konfigurasielêer
commitkit --config /path/to/custom-config.toml

# Uitgebreide afvoer
commitkit --verbose

# Gebruik emoji's in commit-boodskappe
commitkit --emoji

# Gebruik 'n sjabloon vir jou commit
commitkit --template feature

# Dateer CHANGELOG.md outomaties op
commitkit --changelog

# Installeer git-hooks
commitkit --install-hooks

# Wys commit-statistieke vir die afgelope 30 dae
commitkit --stats --days 30

# Valideer 'n commit-boodskap
commitkit --validate .git/COMMIT_EDITMSG
```

## Ontwikkeling

### Bou

```bash
# Ontfoutingsbou
cargo build

# Vrystellingsbou
cargo build --release
```

### Toets

```bash
# Voer alle toetse uit
cargo test

# Voer toetse met afvoer uit
cargo test -- --nocapture
```

### Lint

```bash
# Kontroleer vir lint-kwessies
cargo clippy --all-targets --all-features

# Pas outomatiese regstellings toe
cargo clippy --fix --allow-dirty
```

### Formatering

```bash
# Kontroleer formatering
cargo fmt -- --check

# Pas formatering toe
cargo fmt
```

## Konfigurasie

CommitKit soek na 'n konfigurasielêer genaamd `.commitkit.toml` in die volgende liggings (in volgorde):

1. Huidige gids
2. Gebruiker se tuisgids

Indien geen konfigurasielêer gevind word nie, word 'n versteklêer outomaties in jou tuisgids gegenereer.

Voorbeeldkonfigurasie:

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

## Lisensie

Hierdie projek is gelisensieer onder die MIT-lisensie — sien die [LICENSE](../../../LICENSE)-lêer vir besonderhede.

## Bydraes

Bydraes is welkom! Maak gerus kwessies oop of dien trekversoeke in.

1. Vurk die bewaarplek
2. Skep jou kenmerk-tak (`git checkout -b feature/amazing-feature`)
3. Commit jou wysigings met konvensionele commits
4. Stoot die tak op (`git push origin feature/amazing-feature`)
5. Maak 'n Pull Request oop
