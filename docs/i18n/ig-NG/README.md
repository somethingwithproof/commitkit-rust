[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit bụ ngwa ahịrị iwu na-enyere ndị mmepe sọftụwia aka ịmepụta ozi commit ahaziri nke ọma ma kwụsie ike dịka ụkpụrụ [Conventional Commits](https://www.conventionalcommits.org/) si dị. Ọ na-enye ihu mmekọrịta na-eduzi onye ọrụ nzọụkwụ site na nzọụkwụ n'ịrụ ozi commit nwere ụdị, oke mmetụta (nhọrọ), isiokwu, ahụ ozi, na ụkwụ ozi.

## Atụmatụ

- **Ọrụ Ndị Isi**
  - Nhọrọ mmekọrịta nke ụdị commit na oke mmetụta
  - Nhazi enwere ike ịhazi site na `.commitkit.toml`
  - Mmanye nke oke ogologo isiokwu
  - Nkwado maka ahụ ozi commit nwere ọtụtụ ahịrị
  - Nyochaa izi ezi nke ntinye na-akpaghị aka
  - Nhọrọ commit nwere mbinye aka (`Signed-off-by`)
  - Ndakọrịta n'ọtụtụ sistemụ arụmọrụ

- **Atụmatụ Ebuliri Elu**
  - Nkwado emoji n'ozi commit (✨, 🐛, wdg.)
  - Ndebiri commit nwere nnọchi ebe ndebe
  - Mmepụta na mmelite ndekọ mgbanwe na-akpaghị aka
  - Hooks Git maka inyocha izi ezi na ịhazi ozi commit
  - Ọnụọgụgụ na nyocha nke commit
  - Ajụjụ mmekọrịta maka uru ndebiri

## Ihe Achọrọ Mbụ

- **Rust**: Ụdị 1.88.0 ma ọ bụ karịa (MSRV)
- **Cargo**: Gụnyere na nwụnye Rust

## Nwụnye

### Site na koodu isi mmalite

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Ojiji

Banye n'ime ebe nchekwa Git gị ma gbaa:

```shell
commitkit
```

### Nhọrọ ahịrị iwu

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Wụfee git commit ma bipụta ozi ahụ na stdout
    -v, --verbose           Gosi nkọwa zuru ezu
    -c, --config            Ụzọ faịlụ nhazi (ndabara: .commitkit.toml n'akwụkwọ ndekọ ugbu a ma ọ bụ nke ụlọ)
        --emoji             Gbanwuo ma ọ bụ gbanyụọ emoji n'ozi commit
        --template          Jiri ndebiri commit akọwapụtara
        --changelog         Melite ndekọ mgbanwe mgbe e mechara commit
        --install-hooks     Wụnye hooks nke git
        --stats             Gosi ọnụọgụgụ commit
        --days              Ọnụọgụgụ ụbọchị a ga-enyocha maka ọnụọgụgụ (ndabara: niile)
        --validate          Nyochaa izi ezi nke faịlụ ozi commit
        --prepare-msg       Kwadebe ozi commit (nke hook git na-eji)
        --lang <LANG>       Asụsụ ngosi (dịka en, ig-NG, yo-NG, ha-NG, sw-KE)
    -h, --help              Bipụta ozi enyemaka
    -V, --version           Bipụta ozi ụdị
```

### Asụsụ A Kwadoro

CommitKit na-achọpụta asụsụ ngosi site na `LC_ALL`, `LC_MESSAGES`, na `LANG`, ma ọ bụ nata `--lang <LANG>`. Ọ na-akwado asụsụ 45, gụnyere asụsụ 6 e dere site n'aka nri gaa n'aka ekpe (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, na `ur`). Hụ tebụl zuru ezu na [README Bekee](../../../README.md).

### Usoro mmekọrịta

Ngwa a ga-eduzi gị n'ịmepụta ozi commit dị n'ụkpụrụ ọdịnala:

1. Họrọ ụdị commit (`feat`, `fix`, `docs`, wdg.)
2. Họrọ oke mmetụta (nhọrọ)
3. Tinye isiokwu commit
4. Tinye ahụ ozi commit (nhọrọ)
5. Tinye ụkwụ ozi commit (nhọrọ)

Mgbe ị mechara nzọụkwụ ndị a, CommitKit ga-agba `git commit` na ozi ị rụrụ.

### Ọmụmaatụ

```shell
# Ojiji nkịtị
commitkit

# Ọnọdụ nnwale - gosi ozi commit na-emeghị commit
commitkit --dry-run

# Jiri faịlụ nhazi ahaziri iche
commitkit --config /path/to/custom-config.toml

# Nkọwa zuru ezu
commitkit --verbose

# Jiri emoji n'ozi commit
commitkit --emoji

# Jiri ndebiri maka commit gị
commitkit --template feature

# Melite CHANGELOG.md na-akpaghị aka
commitkit --changelog

# Wụnye hooks git
commitkit --install-hooks

# Gosi ọnụọgụgụ commit maka ụbọchị 30 gara aga
commitkit --stats --days 30

# Nyochaa izi ezi nke ozi commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Mmepe

### Mwube

```bash
# Mwube nchọpụta njehie (Debug)
cargo build

# Mwube ntọhapụ (Release)
cargo build --release
```

### Nnwale

```bash
# Gbaa nnwale niile
cargo test

# Gbaa nnwale na nsonaazụ zuru ezu
cargo test -- --nocapture
```

### Nyocha koodu (Lint)

```bash
# Lelee nsogbu linting
cargo clippy --all-targets --all-features

# Tinye mgbazi na-akpaghị aka
cargo clippy --fix --allow-dirty
```

### Nhazi edemede koodu

```bash
# Lelee nhazi edemede
cargo fmt -- --check

# Tinye nhazi edemede
cargo fmt
```

## Nhazi

CommitKit na-achọ faịlụ nhazi akpọrọ `.commitkit.toml` n'ebe ndị a (n'usoro):

1. Akwụkwọ ndekọ ugbu a
2. Akwụkwọ ndekọ ụlọ nke onye ọrụ

Ọ bụrụ na ahụghị faịlụ nhazi ọ bụla, a ga-emepụta nke ndabara n'akwụkwọ ndekọ ụlọ gị.

Ọmụmaatụ nhazi:

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

## Ikikere

Ọrụ a nwere ikikere n'okpuru Ikikere MIT — lee faịlụ [LICENSE](../../../LICENSE) maka nkọwa.

## Ntinye Aka

A na-anabata ntinye aka! Nwere onwe gị imepe issues ma ọ bụ ziga pull requests.

1. Mee fork nke ebe nchekwa a
2. Mepụta alaka atụmatụ gị (`git checkout -b feature/amazing-feature`)
3. Tinye mgbanwe gị na commit site na iji conventional commits
4. Ziga alaka ahụ (`git push origin feature/amazing-feature`)
5. Mepee Pull Request
