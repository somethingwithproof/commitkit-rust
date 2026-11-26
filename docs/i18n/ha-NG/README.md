[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit kayan aikin layin umarni ne da ke taimaka wa masu haɓaka manhajoji wajen ƙirƙirar saƙonnin commit masu tsari da daidaito bisa ƙa'idar [Conventional Commits](https://www.conventionalcommits.org/). Yana samar da tsarin mu'amala mai jagorantar mai amfani mataki-mataki don haɗa saƙon commit mai ɗauke da nau'i, iyaka (na zaɓi), jigo, jiki, da ƙasan saƙo.

## Fasaloli

- **Babban Aiki**
  - Zaɓin nau'o'in commit da iyakoki cikin mu'amala
  - Saiti mai gyaruwa ta hanyar `.commitkit.toml`
  - Tabbatar da iyakar tsayin jigo
  - Tallafin jikin saƙon commit mai layuka da yawa
  - Tabbatar da ingancin shigarwa ta atomatik
  - Zaɓin ƙara layin `Signed-off-by` a cikin commit
  - Dacewa da tsarin aiki daban-daban

- **Ƙarin Fasaloli**
  - Tallafin emoji a cikin saƙonnin commit (✨, 🐛, da sauransu)
  - Samfuran commit tare da maye gurbin ma'auni
  - Ƙirƙira da sabunta rajistar sauye-sauye ta atomatik
  - Hooks na Git don tabbatarwa da tsara saƙonnin commit
  - Ƙididdigar commit da nazari
  - Tambayoyin mu'amala don ƙimar samfuri

## Abubuwan da ake buƙata

- **Rust**: Sigar 1.88.0 ko mafi girma (MSRV)
- **Cargo**: Yana zuwa tare da shigarwar Rust

## Shigarwa

### Daga lambar tushe

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Amfani

Shiga cikin ma'ajiyar Git ɗinka sannan ka gudanar da:

```shell
commitkit
```

### Zaɓuɓɓukan layin umarni

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Tsallake git commit sannan a buga saƙon zuwa stdout
    -v, --verbose           Nuna cikakken bayanin fitarwa
    -c, --config            Hanyar fayil ɗin saiti (tsoho: .commitkit.toml a cikin babban fayil na yanzu ko na gida)
        --emoji             Kunna ko kashe emoji a cikin saƙonnin commit
        --template          Yi amfani da takamaiman samfurin commit
        --changelog         Sabunta rajistar sauye-sauye bayan commit
        --install-hooks     Shigar da hooks na git
        --stats             Nuna ƙididdigar commit
        --days              Adadin kwanakin da za a yi nazari don ƙididdiga (tsoho: duka)
        --validate          Tabbatar da ingancin fayil ɗin saƙon commit
        --prepare-msg       Shirya saƙon commit (hook na git yana amfani da shi)
        --lang <LANG>       Harshen nuni (misali en, ha-NG, yo-NG, ig-NG, sw-KE)
    -h, --help              Buga bayanin taimako
    -V, --version           Buga bayanin siga
```

### Harsunan da ake tallafawa

CommitKit yana gano harshen nuni ta atomatik daga `LC_ALL`, `LC_MESSAGES`, da `LANG`, ko ya karɓi `--lang <LANG>`. Yana tallafawa harsuna 45, ciki har da harsuna 6 da ake rubutawa daga dama zuwa hagu (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, da `ur`). Duba cikakken teburin a cikin [README na Turanci](../../../README.md).

### Tsarin mu'amala

Kayan aikin zai jagorance ka wajen ƙirƙirar saƙon commit na ƙa'ida:

1. Zaɓi nau'in commit (`feat`, `fix`, `docs`, da sauransu)
2. Zaɓi iyaka (na zaɓi)
3. Shigar da jigon commit
4. Shigar da jikin commit (na zaɓi)
5. Shigar da ƙasan saƙon commit (na zaɓi)

Bayan kammala waɗannan matakai, CommitKit zai aiwatar da `git commit` da saƙon da aka ƙirƙira.

### Misalai

```shell
# Amfani na yau da kullun
commitkit

# Yanayin gwaji - nuna saƙon commit ba tare da yin commit ba
commitkit --dry-run

# Yi amfani da fayil ɗin saiti na musamman
commitkit --config /path/to/custom-config.toml

# Cikakken bayanin fitarwa
commitkit --verbose

# Yi amfani da emoji a cikin saƙonnin commit
commitkit --emoji

# Yi amfani da samfuri don commit ɗinka
commitkit --template feature

# Sabunta CHANGELOG.md ta atomatik
commitkit --changelog

# Shigar da hooks na git
commitkit --install-hooks

# Nuna ƙididdigar commit na kwanaki 30 da suka gabata
commitkit --stats --days 30

# Tabbatar da saƙon commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Haɓakawa

### Gina

```bash
# Ginin gwaji (Debug)
cargo build

# Ginin saki (Release)
cargo build --release
```

### Gwaji

```bash
# Gudanar da dukkan gwaje-gwaje
cargo test

# Gudanar da gwaje-gwaje tare da fitarwa
cargo test -- --nocapture
```

### Binciken lamba (Lint)

```bash
# Duba matsalolin linting
cargo clippy --all-targets --all-features

# Aiwatar da gyare-gyare ta atomatik
cargo clippy --fix --allow-dirty
```

### Tsara lamba

```bash
# Duba tsarin lamba
cargo fmt -- --check

# Aiwatar da tsara lamba
cargo fmt
```

## Saiti

CommitKit yana neman fayil ɗin saiti mai suna `.commitkit.toml` a waɗannan wurare (a jere):

1. Babban fayil na yanzu
2. Babban fayil ɗin gida na mai amfani

Idan ba a sami fayil ɗin saiti ba, za a ƙirƙiri tsoho a cikin babban fayil ɗin gidanka.

Misalin saiti:

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

## Lasisi

Wannan aikin yana ƙarƙashin Lasisin MIT — duba fayil ɗin [LICENSE](../../../LICENSE) don ƙarin bayani.

## Gudummawa

Ana maraba da gudummawa! Kuna iya buɗe issues ko aika pull requests.

1. Yi fork na ma'ajiyar
2. Ƙirƙiri reshen fasalinka (`git checkout -b feature/amazing-feature`)
3. Yi commit na sauye-sauyenka ta amfani da conventional commits
4. Tura reshen (`git push origin feature/amazing-feature`)
5. Buɗe Pull Request
