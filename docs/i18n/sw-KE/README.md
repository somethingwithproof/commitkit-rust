[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit ni zana ya mstari wa amri (CLI) inayowasaidia wasanidi programu kuunda ujumbe wa commit ulio na muundo mzuri na thabiti kwa kufuata kanuni za [Conventional Commits](https://www.conventionalcommits.org/). Inatoa kiolesura shirikishi kinachokuongoza hatua kwa hatua katika kuandaa ujumbe wa commit wenye aina, wigo (si lazima), kichwa, maelezo na sehemu ya chini.

## Vipengele

- **Utendaji wa Msingi**
  - Uchaguzi shirikishi wa aina za commit na wigo
  - Usanidi unaoweza kubinafsishwa kupitia `.commitkit.toml`
  - Udhibiti wa urefu wa juu wa kichwa
  - Msaada wa maelezo ya commit yenye mistari mingi
  - Uthibitishaji otomatiki wa ingizo
  - Hiari ya commit zenye saini (`Signed-off-by`)
  - Utangamano kwenye mifumo mbalimbali ya uendeshaji

- **Vipengele Vilivyoboreshwa**
  - Msaada wa emoji katika ujumbe wa commit (✨, 🐛, n.k.)
  - Violezo vya commit vyenye ubadilishaji wa vishikilia nafasi
  - Uundaji na usasishaji otomatiki wa kumbukumbu ya mabadiliko
  - Hooks za Git za kuthibitisha na kupangilia ujumbe wa commit
  - Takwimu na uchambuzi wa commit
  - Maswali shirikishi ya kuingiza thamani za violezo

## Mahitaji ya Awali

- **Rust**: Toleo la 1.88.0 au jipya zaidi (MSRV)
- **Cargo**: Imejumuishwa na usakinishaji wa Rust

## Usakinishaji

### Kutoka kwa msimbo chanzo

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Matumizi

Nenda kwenye hazina yako ya Git kisha endesha:

```shell
commitkit
```

### Chaguo za mstari wa amri

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Ruka git commit na uchapishe ujumbe kwenye stdout
    -v, --verbose           Onyesha matokeo ya kina
    -c, --config            Njia ya faili ya usanidi (chaguomsingi: .commitkit.toml katika saraka ya sasa au ya nyumbani)
        --emoji             Washa au zima emoji katika ujumbe wa commit
        --template          Tumia kiolezo maalum cha commit
        --changelog         Sasisha kumbukumbu ya mabadiliko baada ya commit
        --install-hooks     Sakinisha hooks za git
        --stats             Onyesha takwimu za commit
        --days              Idadi ya siku za kuchanganua kwa takwimu (chaguomsingi: zote)
        --validate          Thibitisha faili ya ujumbe wa commit
        --prepare-msg       Andaa ujumbe wa commit (hutumiwa na hook ya git)
        --lang <LANG>       Lugha ya kuonyesha (mfano en, sw-KE, am-ET, yo-NG, ja)
    -h, --help              Chapisha maelezo ya msaada
    -V, --version           Chapisha maelezo ya toleo
```

### Lugha Zinazotumika

CommitKit hutambua lugha ya kuonyesha kutoka `LC_ALL`, `LC_MESSAGES` na `LANG`, au hupokea `--lang <LANG>`. Inasaidia lugha 45, ikiwa ni pamoja na lugha 6 zinazoandikwa kutoka kulia kwenda kushoto (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` na `ur`). Tazama jedwali kamili katika [README ya Kiingereza](../../../README.md).

### Mchakato shirikishi

Zana hii itakuongoza katika kuunda ujumbe wa commit wa kawaida:

1. Chagua aina ya commit (`feat`, `fix`, `docs`, n.k.)
2. Chagua wigo (si lazima)
3. Ingiza kichwa cha commit
4. Ingiza maelezo ya commit (si lazima)
5. Ingiza sehemu ya chini ya commit (si lazima)

Baada ya kukamilisha hatua hizi, CommitKit itatekeleza `git commit` kwa ujumbe ulioundwa.

### Mifano

```shell
# Matumizi ya msingi
commitkit

# Hali ya majaribio - onyesha ujumbe wa commit bila kufanya commit
commitkit --dry-run

# Tumia faili maalum ya usanidi
commitkit --config /path/to/custom-config.toml

# Matokeo ya kina
commitkit --verbose

# Tumia emoji katika ujumbe wa commit
commitkit --emoji

# Tumia kiolezo kwa commit yako
commitkit --template feature

# Sasisha CHANGELOG.md kiotomatiki
commitkit --changelog

# Sakinisha hooks za git
commitkit --install-hooks

# Onyesha takwimu za commit za siku 30 zilizopita
commitkit --stats --days 30

# Thibitisha ujumbe wa commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Uendelezaji

### Ujenzi

```bash
# Ujenzi wa utatuzi (Debug)
cargo build

# Ujenzi wa toleo (Release)
cargo build --release
```

### Majaribio

```bash
# Endesha majaribio yote
cargo test

# Endesha majaribio na matokeo kamili
cargo test -- --nocapture
```

### Ukaguzi wa msimbo (Lint)

```bash
# Kagua maonyo ya Clippy
cargo clippy --all-targets --all-features

# Tekeleza marekebisho otomatiki
cargo clippy --fix --allow-dirty
```

### Upangiliaji wa msimbo

```bash
# Kagua upangiliaji
cargo fmt -- --check

# Tekeleza upangiliaji
cargo fmt
```

## Usanidi

CommitKit hutafuta faili ya usanidi inayoitwa `.commitkit.toml` katika maeneo yafuatayo (kwa mpangilio):

1. Saraka ya sasa
2. Saraka ya nyumbani ya mtumiaji

Ikiwa hakuna faili ya usanidi itakayopatikana, faili chaguomsingi itaundwa katika saraka yako ya nyumbani.

Mfano wa usanidi:

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

## Leseni

Mradi huu una leseni chini ya Leseni ya MIT — tazama faili ya [LICENSE](../../../LICENSE) kwa maelezo zaidi.

## Kuchangia

Michango inakaribishwa! Jisikie huru kufungua issues au kuwasilisha pull requests.

1. Fanya fork ya hazina
2. Unda tawi lako la kipengele (`git checkout -b feature/amazing-feature`)
3. Fanya commit ya mabadiliko yako kwa kutumia conventional commits
4. Sukuma tawi (`git push origin feature/amazing-feature`)
5. Fungua Pull Request
