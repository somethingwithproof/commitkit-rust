[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

I-CommitKit iyithuluzi lomugqa womyalo (CLI) elisiza abathuthukisi bakhe imilayezo ye-commit ehleleke kahle futhi ehambisanayo ngokulandela imigomo ye-[Conventional Commits](https://www.conventionalcommits.org/). Inikeza isixhumi esibonakalayo sokusebenzisana esikuqondisa isinyathelo ngesinyathelo ekwakheni umlayezo we-commit onohlobo, ububanzi bokuzithandela, isihloko, umzimba, nonyawo lomlayezo.

## Izici

- **Ukusebenza Okuyisisekelo**
  - Ukukhetha kokusebenzisana kwezinhlobo zama-commit nobubanzi
  - Ukulungiselela okuguqukayo nge-`.commitkit.toml`
  - Ukuqinisekisa ubude obukhulu besihloko
  - Ukusekela imizimba yama-commit enemigqa eminingi
  - Ukuqinisekiswa okuzenzakalelayo kokufakiwe
  - Ukhetho lwama-commit asayiniwe (`Signed-off-by`)
  - Ukusebenza kuzo zonke izinhlelo zokusebenza

- **Izici Ezithuthukisiwe**
  - Ukusekela ama-emoji emilayezweni ye-commit (✨, 🐛, njll.)
  - Amathemplethi e-commit anokushintsha izindawo ezibambile
  - Ukudala nokubuyekeza irekhodi lezinguquko ngokuzenzakalelayo
  - Ama-hooks e-Git okuqinisekisa nokufometha imilayezo ye-commit
  - Izibalo nokuhlaziya kwama-commit
  - Imibuzo yokusebenzisana yamanani ethemplethi

## Okudingekayo Ngaphambilini

- **Rust**: Inguqulo 1.88.0 noma ngaphezulu (MSRV)
- **Cargo**: Ifakiwe ekufakweni kwe-Rust

## Ukufaka

### Kusuka kukhodi yomthombo

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Ukusetshenziswa

Iya endaweni yakho yokugcina ye-Git bese ugijima:

```shell
commitkit
```

### Izinketho zomugqa womyalo

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Yeqa i-git commit bese uphrinta umlayezo ku-stdout
    -v, --verbose           Bonisa umphumela onemininingwane
    -c, --config            Indlela yefayela lokulungiselela (okuzenzakalelayo: .commitkit.toml kufolda yamanje noma yasekhaya)
        --emoji             Vula noma vala ama-emoji emilayezweni ye-commit
        --template          Sebenzisa ithemplethi ethile ye-commit
        --changelog         Buyekeza irekhodi lezinguquko ngemva kwe-commit
        --install-hooks     Faka ama-hooks e-git
        --stats             Bonisa izibalo ze-commit
        --days              Inani lezinsuku zokuhlaziya izibalo (okuzenzakalelayo: zonke)
        --validate          Qinisekisa ifayela lomlayezo we-commit
        --prepare-msg       Lungiselela umlayezo we-commit (isetshenziswa yi-hook ye-git)
        --lang <LANG>       Ulimi lokubonisa (isib. en, zu-ZA, xh-ZA, af-ZA, sw-KE)
    -h, --help              Phrinta ulwazi losizo
    -V, --version           Phrinta ulwazi lwenguqulo
```

### Izilimi Ezisekelwayo

I-CommitKit ithola ulimi lokubonisa ku-`LC_ALL`, `LC_MESSAGES` ne-`LANG`, noma yamukele `--lang <LANG>`. Isekela izilimi ezingu-45, kuhlanganise nezilimi eziyisi-6 ezibhalwa kusuka kwesokudla kuya kwesokunxele (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` ne-`ur`). Bona ithebula eligcwele ku-[README yesiNgisi](../../../README.md).

### Inqubo yokusebenzisana

Ithuluzi lizokuqondisa ekwakheni umlayezo we-commit olandela imithetho:

1. Khetha uhlobo lwe-commit (`feat`, `fix`, `docs`, njll.)
2. Khetha ububanzi (ngokuzithandela)
3. Faka isihloko se-commit
4. Faka umzimba we-commit (ngokuzithandela)
5. Faka unyawo lomlayezo we-commit (ngokuzithandela)

Ngemva kokuqeda lezi zinyathelo, i-CommitKit izosebenzisa `git commit` ngomlayezo owakhile.

### Izibonelo

```shell
# Ukusetshenziswa okuyisisekelo
commitkit

# Imodi yokuhlola - bonisa umlayezo we-commit ngaphandle kokwenza i-commit
commitkit --dry-run

# Sebenzisa ifayela lokulungiselela elenziwe ngokwezifiso
commitkit --config /path/to/custom-config.toml

# Umphumela onemininingwane
commitkit --verbose

# Sebenzisa ama-emoji emilayezweni ye-commit
commitkit --emoji

# Sebenzisa ithemplethi ye-commit yakho
commitkit --template feature

# Buyekeza i-CHANGELOG.md ngokuzenzakalelayo
commitkit --changelog

# Faka ama-hooks e-git
commitkit --install-hooks

# Bonisa izibalo ze-commit zezinsuku ezingu-30 ezedlule
commitkit --stats --days 30

# Qinisekisa umlayezo we-commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Ukuthuthukisa

### Ukwakha

```bash
# Ukwakha kwe-Debug
cargo build

# Ukwakha kwe-Release
cargo build --release
```

### Ukuhlola

```bash
# Qalisa zonke izivivinyo
cargo test

# Qalisa izivivinyo kanye nemiphumela
cargo test -- --nocapture
```

### Ukuhlola ikhodi (Lint)

```bash
# Hlola izexwayiso ze-Clippy
cargo clippy --all-targets --all-features

# Sebenzisa ukulungiswa okuzenzakalelayo
cargo clippy --fix --allow-dirty
```

### Ukufometha

```bash
# Hlola ukufometha
cargo fmt -- --check

# Sebenzisa ukufometha
cargo fmt
```

## Ukulungiselela

I-CommitKit ifuna ifayela lokulungiselela elibizwa nge-`.commitkit.toml` kulezi zindawo (ngokulandelana):

1. Ifolda yamanje
2. Ifolda yasekhaya yomsebenzisi

Uma ingekho ifayela lokulungiselela etholakalayo, kuzokwakhiwa eyokuzenzakalelayo kufolda yakho yasekhaya.

Isibonelo sokulungiselela:

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

## Ilayisensi

Le phrojekthi inelayisensi ngaphansi kweLayisensi ye-MIT — bona ifayela [LICENSE](../../../LICENSE) ngemininingwane.

## Ukufaka Isandla

Iminikelo yamukelekile! Zizwe ukhululekile ukuvula ama-issues noma ukuthumela ama-pull requests.

1. Yenza i-fork yendawo yokugcina
2. Dala igatsha lakho lesici (`git checkout -b feature/amazing-feature`)
3. Yenza i-commit yezinguquko zakho usebenzisa i-conventional commits
4. Phusha igatsha (`git push origin feature/amazing-feature`)
5. Vula i-Pull Request
