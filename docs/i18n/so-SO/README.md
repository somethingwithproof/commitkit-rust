[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit waa qalab khadka taliska (CLI) ah oo ka caawiya horumariyayaasha samaynta farriimaha commit ee habaysan oo joogto ah iyadoo la raacayo halbeegga [Conventional Commits](https://www.conventionalcommits.org/). Waxay bixisaa hab is-dhexgal ah oo tallaabo-tallaabo kuu hagaya marka aad dhisayso farriin commit leh nooc, baaxad ikhtiyaari ah, cinwaan, qoraal dheer iyo qeyb hoose.

## Astaamaha

- **Shaqooyinka Aasaasiga ah**
  - Doorashada is-dhexgalka ee noocyada commit iyo baaxadaha
  - Habayn la beddeli karo iyadoo la adeegsanayo `.commitkit.toml`
  - Xakameynta dhererka ugu badan ee cinwaanka
  - Taageerada qoraalka dheer ee commit ee sadarro badan leh
  - Hubinta tooska ah ee xogta la geliyo
  - Ikhtiyaarka saxiixa commit (`Signed-off-by`)
  - La-qabsiga nidaamyada hawlgalka ee kala duwan

- **Astaamaha Horumarsan**
  - Taageerada emoji ee farriimaha commit (✨, 🐛, iwm.)
  - Qaab-dhismeedyada commit ee beddelka boos-qabsiyada
  - Samaynta iyo cusboonaysiinta tooska ah ee diiwaanka isbeddelka
  - Hooks-ka Git ee hubinta iyo habaynta farriimaha commit
  - Tirakoobka iyo falanqaynta commit
  - Weydiimaha is-dhexgalka ee qiimayaasha qaab-dhismeedka

## Shuruudaha Hore

- **Rust**: Nooca 1.88.0 ama ka sarreeya (MSRV)
- **Cargo**: Waxay la socotaa rakibidda Rust

## Rakibidda

### Laga soo bilaabo koodka isha

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## Isticmaalka

Tag kaydkaaga Git kadibna socodsii:

```shell
commitkit
```

### Ikhtiyaarada khadka taliska

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           Ka bood git commit oo farriinta ku daabac stdout
    -v, --verbose           Muuji natiijo faahfaahsan
    -c, --config            Jihada faylka habaynta (caadi: .commitkit.toml ee galka hadda ama galka guriga)
        --emoji             Daar ama dami emoji ee farriimaha commit
        --template          Adeegso qaab-dhismeed commit oo gaar ah
        --changelog         Cusboonaysii diiwaanka isbeddelka ka dib commit
        --install-hooks     Ku rakib hooks-ka git
        --stats             Muuji tirakoobka commit
        --days              Tirada maalmaha loo falanqaynayo tirakoobka (caadi: dhammaan)
        --validate          Hubi saxnaanta faylka farriinta commit
        --prepare-msg       Diyaari farriinta commit (waxaa adeegsada hook-ga git)
        --lang <LANG>       Luqadda muujinta (tusaale en, so-SO, sw-KE, ar, ja)
    -h, --help              Daabac macluumaadka caawinta
    -V, --version           Daabac macluumaadka nooca
```

### Luqadaha la taageero

CommitKit waxay si toos ah uga ogaataa luqadda muujinta `LC_ALL`, `LC_MESSAGES` iyo `LANG`, ama waxay aqbashaa `--lang <LANG>`. Waxay taageertaa 45 luqadood, oo ay ku jiraan 6 luqadood oo midig-ilaa-bidix loo qoro (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` iyo `ur`). Ka eeg shaxda dhammaystiran [README-ga Ingiriisiga](../../../README.md).

### Habka is-dhexgalka

Qalabku wuxuu kuu hagi doonaa samaynta farriinta commit ee caadiga ah:

1. Dooro nooca commit (`feat`, `fix`, `docs`, iwm.)
2. Dooro baaxadda (ikhtiyaari)
3. Geli cinwaanka commit
4. Geli qoraalka dheer ee commit (ikhtiyaari)
5. Geli qeybta hoose ee commit (ikhtiyaari)

Marka aad dhammayso tallaabooyinkan, CommitKit wuxuu fulin doonaa `git commit` isagoo adeegsanaya farriinta aad dhistay.

### Tusaalooyin

```shell
# Isticmaal aasaasi ah
commitkit

# Habka tijaabada - muuji farriinta commit adigoon samayn commit
commitkit --dry-run

# Adeegso fayl habayn gaar ah
commitkit --config /path/to/custom-config.toml

# Natiijo faahfaahsan
commitkit --verbose

# Adeegso emoji farriimaha commit
commitkit --emoji

# Adeegso qaab-dhismeed commit-kaaga
commitkit --template feature

# Si toos ah u cusboonaysii CHANGELOG.md
commitkit --changelog

# Ku rakib hooks-ka git
commitkit --install-hooks

# Muuji tirakoobka commit ee 30-kii maalmood ee la soo dhaafay
commitkit --stats --days 30

# Hubi saxnaanta farriin commit
commitkit --validate .git/COMMIT_EDITMSG
```

## Horumarinta

### Dhisidda

```bash
# Dhisidda tijaabada (Debug)
cargo build

# Dhisidda sii-daynta (Release)
cargo build --release
```

### Tijaabinta

```bash
# Socodsii dhammaan tijaabooyinka
cargo test

# Socodsii tijaabooyinka adigoo muujinaya natiijada
cargo test -- --nocapture
```

### Hubinta koodka (Lint)

```bash
# Hubi digniinaha Clippy
cargo clippy --all-targets --all-features

# Codso hagaajinta tooska ah
cargo clippy --fix --allow-dirty
```

### Qaabaynta

```bash
# Hubi qaabaynta koodka
cargo fmt -- --check

# Codso qaabaynta koodka
cargo fmt
```

## Habaynta

CommitKit waxay ka raadisaa faylka habaynta ee `.commitkit.toml` goobahan (sida ay u kala horreeyaan):

1. Galka hadda
2. Galka guriga ee isticmaalaha

Haddii aan la helin fayl habayn, waxaa si toos ah loogu abuurayaa fayl caadi ah galkaaga guriga.

Tusaalaha habaynta:

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

## Shatiga

Mashruucan waxaa lagu siiyay Shatiga MIT — eeg faylka [LICENSE](../../../LICENSE) wixii faahfaahin ah.

## Ka-qaybgalka

Ka-qaybgalka waa la soo dhaweynayaa! Xor u noqo inaad furto issues ama aad soo dirto pull requests.

1. Samee fork kaydkan
2. Abuur laanta astaantaada (`git checkout -b feature/amazing-feature`)
3. Samee commit isbeddelladaada adigoo adeegsanaya conventional commits
4. Dir laanta (`git push origin feature/amazing-feature`)
5. Fur Pull Request
