[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit dia fitaovana amin'ny tsipika baiko manampy ny mpamorona rindrambaiko hanoratra hafatra commit mirindra sy manara-penitra araka ny fepetra [Conventional Commits](https://www.conventionalcommits.org/). Manome sehatra ifanakalozana izy io mba hitarihana tsikelikely ny mpampiasa hanangana hafatra commit ahitana karazana, sehatra (tsy voatery), lohateny, votoaty ary fanamarihana ambany.

## Endri-javatra

- **Fiasa Fototra**
  - Fisafidianana ifanakalozana ny karazana commit sy ny sehatra
  - Fanefena azo amboarina amin'ny alalan'ny `.commitkit.toml`
  - Fanaraha-maso ny halavan'ny lohateny farany ambony
  - Fanohanana ny votoatin'ny commit misy andalana maromaro
  - Fanamarinana mandeha ho azy ny fampidirana
  - Andalan-tsoratra `Signed-off-by` tsy voatery amin'ny commit
  - Mifanaraka amin'ny rafitra fandidiana samihafa

- **Endri-javatra Fanampiny**
  - Fanohanana emoji ao amin'ny hafatra commit (✨, 🐛, sns.)
  - Lasitra commit misy fanoloana sanda
  - Famoronana sy fanavaozana mandeha ho azy ny lisitry ny fiovana
  - Hooks Git ho fanamarinana sy fanamboarana ny hafatra commit
  - Antontan'isa sy fanadihadiana momba ny commit
  - Fanontaniana ifanakalozana ho an'ny sandan'ny lasitra

## Fepetra takiana mialoha

- **Rust**: Kinova 1.88.0 na ambony kokoa (MSRV)
- **Cargo**: Tafiditra ao anatin'ny fametrahana Rust

## Fametrahana

### Avy amin'ny kaody loharano

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Fampiasana

Midira ao amin'ny tahiry Git anao ary tanteraho:

```shell
commitkit
```

### Safidy amin'ny tsipika baiko

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Avelao ny git commit ary asehoy eo amin'ny stdout ny hafatra
    -v, --verbose           Asehoy ny vokatra amin'ny antsipiriany
    -c, --config            Lalan'ny rakitra fanefena (tsotra: .commitkit.toml ao amin'ny lahatahiry ankehitriny na fandraisana)
        --emoji             Ampiasao na tsia ny emoji amin'ny hafatra commit
        --template          Ampiasao ny lasitra commit voafaritra
        --changelog         Havaozy ny lisitry ny fiovana aorian'ny commit
        --install-hooks     Apetraho ny hooks git
        --stats             Asehoy ny antontan'isa momba ny commit
        --days              Isan'ny andro hanaovana antontan'isa (tsotra: rehetra)
        --validate          Hamarino ny rakitra misy hafatra commit
        --prepare-msg       Omano ny hafatra commit (ampiasain'ny hook git)
        --lang <LANG>       Fiteny fampisehoana (oh. en, mg-MG, fr, sw-KE, ja)
    -h, --help              Asehoy ny torohevitra fanampiana
    -V, --version           Asehoy ny kinova
```

### Fiteny tohanana

Mamantatra ho azy ny fiteny fampisehoana avy amin'ny `LC_ALL`, `LC_MESSAGES` ary `LANG` ny CommitKit, na manaiky `--lang <LANG>`. Manohana fiteny 45 izy io, ao anatin'izany ny fiteny 6 soratana avy ankavanana miankavia (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` ary `ur`). Jereo ny lisitra feno ao amin'ny [README amin'ny teny Anglisy](../../../README.md).

### Dingana ifanakalozana

Hitari-dalana anao amin'ny fanoratana hafatra commit manaraka ny fenitra ity fitaovana ity:

1. Safidio ny karazana commit (`feat`, `fix`, `docs`, sns.)
2. Safidio ny sehatra (tsy voatery)
3. Ampidiro ny lohatenin'ny commit
4. Ampidiro ny votoatin'ny commit (tsy voatery)
5. Ampidiro ny fanamarihana ambany (tsy voatery)

Rehefa vita ireo dingana ireo dia hanatanteraka `git commit` amin'ny hafatra noforoninao ny CommitKit.

### Ohatra

```shell
# Fampiasana fototra
commitkit

# Fanandramana - asehoy ny hafatra commit nefa tsy manao commit
commitkit --dry-run

# Mampiasa rakitra fanefena manokana
commitkit --config /path/to/custom-config.toml

# Vokatra amin'ny antsipiriany
commitkit --verbose

# Mampiasa emoji ao amin'ny hafatra commit
commitkit --emoji

# Mampiasa lasitra ho an'ny commit
commitkit --template feature

# Fanavaozana mandeha ho azy ny CHANGELOG.md
commitkit --changelog

# Fametrahana ny hooks git
commitkit --install-hooks

# Asehoy ny antontan'isan'ny commit tao anatin'ny 30 andro farany
commitkit --stats --days 30

# Fanamarinana hafatra commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Fampivoarana

### Fananganana

```bash
# Fananganana fanamarinana (Debug)
cargo build

# Fananganana famoahana (Release)
cargo build --release
```

### Fitsapana

```bash
# Tanteraho ny fitsapana rehetra
cargo test

# Tanteraho ny fitsapana miaraka amin'ny fampisehoana vokatra
cargo test -- --nocapture
```

### Fanamarinana kaody (Lint)

```bash
# Jereo ny fampitandremana Clippy
cargo clippy --all-targets --all-features

# Ampiharo ny fanitsiana mandeha ho azy
cargo clippy --fix --allow-dirty
```

### Fandrafetana kaody

```bash
# Hamarino ny fandrafetana
cargo fmt -- --check

# Ampiharo ny fandrafetana
cargo fmt
```

## Fanefena

Mitady rakitra fanefena antsoina hoe `.commitkit.toml` amin'ireto toerana ireto (arakaraka ny filaharany) ny CommitKit:

1. Lahatahiry ankehitriny
2. Lahatahiry fandraisan'ny mpampiasa

Raha tsy misy rakitra fanefena hita dia hisy rakitra tsotra hoforonina ao amin'ny lahatahiry fandraisanao.

Ohatra fanefena:

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

## Lisansa

Ity tetikasa ity dia omena eo ambanin'ny Lisansa MIT — jereo ny rakitra [LICENSE](../../../LICENSE) raha mila fanazavana fanampiny.

## Fandraisana anjara

Raisina an-tanan-droa ny fandraisana anjara! Aza misalasala manokatra issue na mandefa pull request.

1. Ataovy fork ity tahiry ity
2. Mamorona sampana ho an'ny endri-javatra vaovao (`git checkout -b feature/amazing-feature`)
3. Ataovy commit ny fiovanao araka ny conventional commits
4. Alefaso ny sampana (`git push origin feature/amazing-feature`)
5. Sokafy ny Pull Request
