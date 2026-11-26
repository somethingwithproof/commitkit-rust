[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

I-CommitKit sisixhobo somgca womyalelo esinceda abaphuhlisi benze imiyalezo ye-commit ecwangciswe kakuhle neyangqinelanayo ngokulandela umgaqo we-[Conventional Commits](https://www.conventionalcommits.org/). Inikezela ngojongano olusebenzisanayo olukhokela umsebenzisi inyathelo ngenyathelo ekwakheni umyalezo we-commit onohlobo, umda wokuzithandela, isihloko, umzimba nombhalo ongezantsi.

## Iimpawu

- **Imisebenzi Engundoqo**
  - Ukukhetha okusebenzisanayo kweentlobo zee-commit nemida
  - Uqwalaselo olulungelelaniswayo nge-`.commitkit.toml`
  - Ukunyanzeliswa kobude obuphezulu besihloko
  - Inkxaso yemibhalo yomzimba we-commit enemigca emininzi
  - Ukuqinisekiswa okuzenzekelayo kwegalelo
  - Umgca okhethekayo we-`Signed-off-by` kwii-commit
  - Ukuhambelana namaqonga ahlukeneyo

- **Iimpawu Eziphuculweyo**
  - Inkxaso yee-emoji kwimiyalezo ye-commit (✨, 🐛, njl.)
  - Iitemplate ze-commit ezinokutshintshwa kweendawo ezibambeleyo
  - Ukudalwa nokuhlaziywa okuzenzekelayo kwerekhodi leenguqu
  - Ii-hooks ze-Git zokuqinisekisa nokufomatha imiyalezo ye-commit
  - Amanani nohlalutyo lwee-commit
  - Izikhokelo ezisebenzisanayo zamaxabiso e-template

## Izinto Ezifunekayo Kuqala

- **Rust**: Inguqulelo ye-1.88.0 okanye ngaphezulu (MSRV)
- **Cargo**: Ibandakanyiwe kufakelo lwe-Rust

## Ufakelo

### Ukusuka kwikhowudi yemvelaphi

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Ukusetyenziswa

Yiya kwindawo yakho yogcino ye-Git uze usebenzise:

```shell
commitkit
```

### Iinketho zomgca womyalelo

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Tsiba i-git commit uze uprinte umyalezo kwi-stdout
    -v, --verbose           Bonisa isiphumo esineenkcukacha
    -c, --config            Indlela yefayile yoqwalaselo (okungagqibekanga: .commitkit.toml kwifolda yangoku okanye yasekhaya)
        --emoji             Layita okanye cima ii-emoji kwimiyalezo ye-commit
        --template          Sebenzisa itemplate ethile ye-commit
        --changelog         Hlaziya irekhodi leenguqu emva kwe-commit
        --install-hooks     Faka ii-hooks ze-git
        --stats             Bonisa amanani ee-commit
        --days              Inani leentsuku zokuhlalutya amanani (okungagqibekanga: zonke)
        --validate          Qinisekisa ifayile yomyalezo we-commit
        --prepare-msg       Lungiselela umyalezo we-commit (isetyenziswa yi-hook ye-git)
        --lang <LANG>       Ulwimi lokubonisa (umz. en, xh-ZA, zu-ZA, af-ZA, sw-KE)
    -h, --help              Printa ulwazi loncedo
    -V, --version           Printa ulwazi lwenguqulelo
```

### Iilwimi Ezixhaswayo

I-CommitKit ibona ulwimi lokubonisa kwi-`LC_ALL`, `LC_MESSAGES` ne-`LANG`, okanye yamkela `--lang <LANG>`. Ixhasa iilwimi ezingama-45, kuquka iilwimi ezi-6 ezibhalwa ukusuka ekunene ukuya ekhohlo (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` ne-`ur`). Jonga itheyibhile epheleleyo kwi-[README yesiNgesi](../../../README.md).

### Inkqubo esebenzisanayo

Isixhobo siya kukukhokela ekudaleni umyalezo we-commit olandela imigaqo:

1. Khetha uhlobo lwe-commit (`feat`, `fix`, `docs`, njl.)
2. Khetha umda (ngokuzithandela)
3. Ngenisa isihloko se-commit
4. Ngenisa umzimba we-commit (ngokuzithandela)
5. Ngenisa umbhalo ongezantsi we-commit (ngokuzithandela)

Emva kokugqiba la manyathelo, i-CommitKit iya kuqhuba i-`git commit` ngomyalezo wakho owakhiweyo.

### Imizekelo

```shell
# Ukusetyenziswa okusisiseko
commitkit

# Imowudi yovavanyo - bonisa umyalezo we-commit ngaphandle kokwenza i-commit
commitkit --dry-run

# Sebenzisa ifayile yoqwalaselo elungelelanisiweyo
commitkit --config /path/to/custom-config.toml

# Isiphumo esineenkcukacha
commitkit --verbose

# Sebenzisa ii-emoji kwimiyalezo ye-commit
commitkit --emoji

# Sebenzisa itemplate kwi-commit yakho
commitkit --template feature

# Hlaziya i-CHANGELOG.md ngokuzenzekelayo
commitkit --changelog

# Faka ii-hooks ze-git
commitkit --install-hooks

# Bonisa amanani ee-commit kwiintsuku ezingama-30 ezidlulileyo
commitkit --stats --days 30

# Qinisekisa umyalezo we-commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Uphuhliso

### Ukwakha

```bash
# Ukwakha kwe-Debug
cargo build

# Ukwakha kwe-Release
cargo build --release
```

### Uvavanyo

```bash
# Qhuba zonke iimvavanyo
cargo test

# Qhuba iimvavanyo nesiphumo
cargo test -- --nocapture
```

### Uhlolo lwekhowudi (Lint)

```bash
# Jonga izilumkiso ze-Clippy
cargo clippy --all-targets --all-features

# Faka izilungiso ezizenzekelayo
cargo clippy --fix --allow-dirty
```

### Ukufomatha

```bash
# Jonga ukufomatha
cargo fmt -- --check

# Faka ukufomatha
cargo fmt
```

## Uqwalaselo

I-CommitKit ikhangela ifayile yoqwalaselo ebizwa ngokuba yi-`.commitkit.toml` kwezi ndawo zilandelayo (ngokulandelelana):

1. Ifolda yangoku
2. Ifolda yasekhaya yomsebenzisi

Ukuba akukho fayile yoqwalaselo ifumanekayo, kuya kwenziwa eyokungagqibekanga kwifolda yakho yasekhaya.

Umzekelo woqwalaselo:

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

Le projekthi inelayisensi phantsi kweLayisensi ye-MIT — jonga ifayile ye-[LICENSE](../../../LICENSE) ukuze ufumane iinkcukacha.

## Ukufaka Igalelo

Amagalelo amkelekile! Zive ukhululekile ukuvula ii-issues okanye uthumele ii-pull requests.

1. Yenza i-fork yendawo yogcino
2. Yenza isebe lakho lophawu (`git checkout -b feature/amazing-feature`)
3. Yenza i-commit yeenguqu zakho usebenzisa i-conventional commits
4. Tyhala isebe (`git push origin feature/amazing-feature`)
5. Vula i-Pull Request
