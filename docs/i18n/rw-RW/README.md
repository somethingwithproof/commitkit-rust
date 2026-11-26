[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit ni igikoresho cyo ku murongo w'amabwiriza (CLI) gifasha abahanga mu ikoranabuhanga kwandika ubutumwa bwa commit bufite gahunda kandi buhuye n'amabwiriza ya [Conventional Commits](https://www.conventionalcommits.org/). Gitanga uburyo bw'ibiganiro bukuyobora intambwe ku yindi mu gukora ubutumwa burimo ubwoko, urwego rw'impinduka (si itegeko), umutwe, igihimba n'igice cyo hasi.

## Ibiranga

- **Imikorere y'ibanze**
  - Guhitamo ubwoko bwa commit n'inzego mu buryo bw'ibiganiro
  - Igenamiterere rishobora guhindurwa binyuze muri `.commitkit.toml`
  - Kugenzura uburebure ntarengwa bw'umutwe
  - Gushyigikira igihimba cya commit gifite imirongo myinshi
  - Igenzura ryikora ry'ibyinjiye
  - Umurongo wa `Signed-off-by` ku bushake muri commit
  - Gukora kuri sisitemu z'imikorere zitandukanye

- **Ibiranga byongereweho**
  - Gushyigikira emoji mu butumwa bwa commit (✨, 🐛, n'ibindi)
  - Inyandikorugero za commit zisimbura ibimenyetso
  - Gukora no kuvugurura urutonde rw'impinduka mu buryo bwikora
  - Hooks za Git zo kugenzura no gutunganya ubutumwa bwa commit
  - Imibare n'isesengura rya commit
  - Ibibazo by'ibiganiro byo kuzuza agaciro k'inyandikorugero

## Ibisabwa mbere

- **Rust**: Verisiyo 1.88.0 cyangwa hejuru (MSRV)
- **Cargo**: Iri mu iyinjizwa rya Rust

## Gushyiramo

### Uhereye kuri kode y'inkomoko

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Uko ikoreshwa

Jya mu bubiko bwawe bwa Git maze ukoreshe:

```shell
commitkit
```

### Amahitamo yo ku murongo w'amabwiriza

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Simbuka git commit maze wandike ubutumwa kuri stdout
    -v, --verbose           Erekana ibisohoka birambuye
    -c, --config            Inzira y'idosiye y'igenamiterere (ibanziriza: .commitkit.toml mu bubiko buriho cyangwa ubw'umukoresha)
        --emoji             Gufungura cyangwa gufunga emoji mu butumwa bwa commit
        --template          Koresha inyandikorugero ya commit yihariye
        --changelog         Vugurura urutonde rw'impinduka nyuma ya commit
        --install-hooks     Shyiramo hooks za git
        --stats             Erekana imibare ya commit
        --days              Umubare w'iminsi yo gusesengura mu mibare (ibanziriza: yose)
        --validate          Genzura idosiye y'ubutumwa bwa commit
        --prepare-msg       Tegura ubutumwa bwa commit (bukoreshwa na hook ya git)
        --lang <LANG>       Ururimi rw'imyerekanire (urugero en, rw-RW, sw-KE, fr, ja)
    -h, --help              Erekana ubufasha
    -V, --version           Erekana amakuru ya verisiyo
```

### Indimi zishyigikiwe

CommitKit imenyera ururimi rw'imyerekanire muri `LC_ALL`, `LC_MESSAGES` na `LANG`, cyangwa ikemera `--lang <LANG>`. Ishyigikira indimi 45, harimo indimi 6 zandikwa ziva iburyo zijya ibumoso (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` na `ur`). Reba urutonde rwuzuye muri [README y'Icyongereza](../../../README.md).

### Uburyo bw'ibiganiro

Igikoresho kizakuyobora mu gukora ubutumwa bwa commit bukurikiza amabwiriza:

1. Hitamo ubwoko bwa commit (`feat`, `fix`, `docs`, n'ibindi)
2. Hitamo urwego rw'impinduka (si itegeko)
3. Andika umutwe wa commit
4. Andika igihimba cya commit (si itegeko)
5. Andika igice cyo hasi cya commit (si itegeko)

Nurangiza izi ntambwe, CommitKit izahita ikora `git commit` ikoresheje ubutumwa wateguye.

### Ingero

```shell
# Ikoreshwa risanzwe
commitkit

# Uburyo bw'igerageza - erekanisha ubutumwa bwa commit udakoze commit
commitkit --dry-run

# Koresha idosiye y'igenamiterere yihariye
commitkit --config /path/to/custom-config.toml

# Ibisohoka birambuye
commitkit --verbose

# Koresha emoji mu butumwa bwa commit
commitkit --emoji

# Koresha inyandikorugero kuri commit yawe
commitkit --template feature

# Kuvugurura CHANGELOG.md mu buryo bwikora
commitkit --changelog

# Gushyiramo hooks za git
commitkit --install-hooks

# Erekana imibare ya commit mu minsi 30 ishize
commitkit --stats --days 30

# Kugenzura ubutumwa bwa commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Iterambere

### Kubaka

```bash
# Kubaka ko kugenzura (Debug)
cargo build

# Kubaka ko gusohora (Release)
cargo build --release
```

### Igerageza

```bash
# Koresha ibizamini byose
cargo test

# Koresha ibizamini hamwe n'ibisohoka
cargo test -- --nocapture
```

### Igenzura rya kode (Lint)

```bash
# Genzura ibibazo bya Clippy
cargo clippy --all-targets --all-features

# Shiraho ibikosorwa mu buryo bwikora
cargo clippy --fix --allow-dirty
```

### Gutunganya imyandikire

```bash
# Genzura imyandikire ya kode
cargo fmt -- --check

# Tunganya imyandikire ya kode
cargo fmt
```

## Igenamiterere

CommitKit ishaka idosiye y'igenamiterere yitwa `.commitkit.toml` aha hakurikira (ku murongo):

1. Ububiko buriho
2. Ububiko bw'umukoresha

Niba nta dosiye y'igenamiterere ibonetse, hazakorwa iy'ibanziriza mu bubiko bw'umukoresha wawe.

Urugero rw'igenamiterere:

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

## Uruhushya

Uyu mushinga ufite Uruhushya rwa MIT — reba idosiye [LICENSE](../../../LICENSE) kugira ngo ubone ibisobanuro birambuye.

## Gutanga umusanzu

Umusanzu wawe urakirwa! Ushobora gufungura issues cyangwa ukohereza pull requests.

1. Kora fork y'ububiko
2. Kora ishami ry'igikorwa gishya (`git checkout -b feature/amazing-feature`)
3. Kora commit y'impinduka zawe ukoresheje conventional commits
4. Ohereza ishami (`git push origin feature/amazing-feature`)
5. Fungura Pull Request
