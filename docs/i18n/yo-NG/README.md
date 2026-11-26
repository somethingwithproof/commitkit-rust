[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit jẹ́ ohun èlò ìlà àṣẹ (CLI) tí ó ń ran àwọn olùgbékalẹ̀ sọ́fítíwẹ́ẹ̀rì lọ́wọ́ láti ṣẹ̀dá àwọn ìfiránṣẹ́ commit tí ó wà ní ètò tí ó sì dúró ṣinṣin ní ìbámu pẹ̀lú ìlànà [Conventional Commits](https://www.conventionalcommits.org/). Ó pèsè ìfọ̀rọ̀wánilẹ́nuwò alájọṣepọ̀ tí ó ń darí olùlò lẹ́sẹẹsẹ láti kọ ìfiránṣẹ́ commit pẹ̀lú irú, àgbègbè iṣẹ́ (kì í ṣe dandan), àkọlé, ara ìfiránṣẹ́, àti ẹsẹ̀ ìfiránṣẹ́.

## Àwọn Àǹfààní

- **Iṣẹ́ Àkọ́kọ́**
  - Yíyan irú commit àti àgbègbè iṣẹ́ ní ọ̀nà alájọṣepọ̀
  - Ìṣètò tí a lè yí padà nípasẹ̀ `.commitkit.toml`
  - Ìdánilójú gígùn àkọlé tí ó pọ̀ jù lọ
  - Àtìlẹ́yìn fún ara ìfiránṣẹ́ commit oní-ìlà púpọ̀
  - Àyẹ̀wò ìjẹ́òótọ́ fúnra rẹ̀ fún ìtẹ̀síwájú
  - Àṣàyàn láti fi ìlà `Signed-off-by` kún commit
  - Ìbámu pẹ̀lú àwọn ẹ̀rọ ìṣiṣẹ́ oríṣiríṣi

- **Àwọn Àǹfààní Àfikún**
  - Àtìlẹ́yìn emoji nínú àwọn ìfiránṣẹ́ commit (✨, 🐛, àti bẹ́ẹ̀ bẹ́ẹ̀ lọ)
  - Àwọn àwòkọ́ṣe commit pẹ̀lú ìrọ́pò àyè
  - Ṣíṣẹ̀dá àti ṣíṣe àtúnṣe àkọsílẹ̀ àyípadà fúnra rẹ̀
  - Àwọn hook Git fún ṣíṣàyẹ̀wò àti ṣíṣètò àwọn ìfiránṣẹ́ commit
  - Àwọn ìṣirò àti ìtúpalẹ̀ commit
  - Àwọn ìbéèrè alájọṣepọ̀ fún ìníyelórí àwòkọ́ṣe

## Ohun Tí A Nílò Tẹ́lẹ̀

- **Rust**: Ẹ̀yà 1.88.0 tàbí jù bẹ́ẹ̀ lọ (MSRV)
- **Cargo**: Wà pẹ̀lú ìfisori ẹ̀rọ Rust

## Ìfisori Ẹ̀rọ

### Láti inú kóòdù orísun

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Lílo

Lọ sí ibi ìpamọ́ Git rẹ kí o sì ṣiṣẹ́:

```shell
commitkit
```

### Àwọn àṣàyàn ìlà àṣẹ

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Fò git commit kọjá kí o sì tẹ ìfiránṣẹ́ náà sí stdout
    -v, --verbose           Ṣàfihàn àbájáde kíkún
    -c, --config            Ọ̀nà sí fáìlì ìṣètò (àkọ́yàn: .commitkit.toml nínú fódà lọ́wọ́lọ́wọ́ tàbí ti ilé)
        --emoji             Tan tàbí pa emoji nínú àwọn ìfiránṣẹ́ commit
        --template          Lo àwòkọ́ṣe commit pàtó kan
        --changelog         Ṣe àtúnṣe àkọsílẹ̀ àyípadà lẹ́yìn commit
        --install-hooks     Fi àwọn hook git sori ẹ̀rọ
        --stats             Ṣàfihàn àwọn ìṣirò commit
        --days              Iye ọjọ́ láti ṣàyẹ̀wò fún ìṣirò (àkọ́yàn: gbogbo rẹ̀)
        --validate          Ṣàyẹ̀wò ìjẹ́òótọ́ fáìlì ìfiránṣẹ́ commit
        --prepare-msg       Pèsè ìfiránṣẹ́ commit sílẹ̀ (tí hook git ń lò)
        --lang <LANG>       Èdè ìfihàn (fún àpẹẹrẹ en, yo-NG, ig-NG, ha-NG, sw-KE)
    -h, --help              Tẹ ìròyìn ìrànlọ́wọ́ jáde
    -V, --version           Tẹ ìròyìn ẹ̀yà jáde
```

### Àwọn Èdè Tí A Ṣètìlẹ́yìn Fún

CommitKit ń ṣàwárí èdè ìfihàn fúnra rẹ̀ láti `LC_ALL`, `LC_MESSAGES`, àti `LANG`, tàbí kí ó gba `--lang <LANG>`. Ó ń ṣètìlẹ́yìn fún èdè 45, pẹ̀lú èdè 6 tí a ń kọ láti ọ̀tún sí òsì (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, àti `ur`). Wo tábìlì kíkún nínú [README èdè Gẹ̀ẹ́sì](../../../README.md).

### Ìlànà alájọṣepọ̀

Ohun èlò yìí yóò darí rẹ nínú ṣíṣẹ̀dá ìfiránṣẹ́ commit tí ó bá ìlànà mu:

1. Yan irú commit (`feat`, `fix`, `docs`, àti bẹ́ẹ̀ bẹ́ẹ̀ lọ)
2. Yan àgbègbè iṣẹ́ (kì í ṣe dandan)
3. Tẹ àkọlé commit síi
4. Tẹ ara ìfiránṣẹ́ commit síi (kì í ṣe dandan)
5. Tẹ ẹsẹ̀ ìfiránṣẹ́ commit síi (kì í ṣe dandan)

Lẹ́yìn tí o bá parí àwọn ìgbésẹ̀ wọ̀nyí, CommitKit yóò ṣe `git commit` pẹ̀lú ìfiránṣẹ́ tí o kọ.

### Àwọn Àpẹẹrẹ

```shell
# Lílo ìpìlẹ̀
commitkit

# Ipò àyẹ̀wò - ṣàfihàn ìfiránṣẹ́ commit láìṣe commit
commitkit --dry-run

# Lo fáìlì ìṣètò àdáni
commitkit --config /path/to/custom-config.toml

# Àbájáde kíkún
commitkit --verbose

# Lo emoji nínú àwọn ìfiránṣẹ́ commit
commitkit --emoji

# Lo àwòkọ́ṣe fún commit rẹ
commitkit --template feature

# Ṣe àtúnṣe CHANGELOG.md fúnra rẹ̀
commitkit --changelog

# Fi àwọn hook git sori ẹ̀rọ
commitkit --install-hooks

# Ṣàfihàn àwọn ìṣirò commit fún ọjọ́ 30 tí ó kọjá
commitkit --stats --days 30

# Ṣàyẹ̀wò ìjẹ́òótọ́ ìfiránṣẹ́ commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Ìdàgbàsókè

### Ìkọ́lé

```bash
# Ìkọ́lé àyẹ̀wò (Debug)
cargo build

# Ìkọ́lé ìtúsílẹ̀ (Release)
cargo build --release
```

### Ìdánwò

```bash
# Ṣe gbogbo ìdánwò
cargo test

# Ṣe ìdánwò pẹ̀lú àbájáde kíkún
cargo test -- --nocapture
```

### Àyẹ̀wò kóòdù (Lint)

```bash
# Ṣàyẹ̀wò àwọn ìkìlọ̀ Clippy
cargo clippy --all-targets --all-features

# Lo àwọn àtúnṣe aládàáṣe
cargo clippy --fix --allow-dirty
```

### Ìṣètò kóòdù

```bash
# Ṣàyẹ̀wò ìṣètò
cargo fmt -- --check

# Lo ìṣètò
cargo fmt
```

## Ìṣètò

CommitKit ń wá fáìlì ìṣètò tí a ń pè ní `.commitkit.toml` ní àwọn ibi wọ̀nyí (lẹ́sẹẹsẹ):

1. Fódà lọ́wọ́lọ́wọ́
2. Fódà ilé ti olùlò

Tí a kò bá rí fáìlì ìṣètò kankan, a ó ṣẹ̀dá àkọ́yàn kan nínú fódà ilé rẹ.

Àpẹẹrẹ ìṣètò:

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

## Ìwé-àṣẹ

Iṣẹ́ yìí wà lábẹ́ Ìwé-àṣẹ MIT — wo fáìlì [LICENSE](../../../LICENSE) fún ẹ̀kúnrẹ́rẹ́ àlàyé.

## Ìkópa

A ń gba ìkópa tọwọ́tẹsẹ̀! Má ṣe lọ́ra láti ṣí issues tàbí fi pull requests ránṣẹ́.

1. Ṣe fork ibi ìpamọ́ náà
2. Ṣẹ̀dá ẹ̀ka àǹfààní rẹ (`git checkout -b feature/amazing-feature`)
3. Ṣe commit àwọn àyípadà rẹ nípa lílo conventional commits
4. Ti ẹ̀ka náà sókè (`git push origin feature/amazing-feature`)
5. Ṣí Pull Request
