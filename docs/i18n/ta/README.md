[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit என்பது [Conventional Commits](https://www.conventionalcommits.org/) விவரக்குறிப்பைப் பின்பற்றி ஒழுங்கமைக்கப்பட்ட மற்றும் சீரான கமிட் செய்திகளை உருவாக்க மென்பொருள் உருவாக்குநர்களுக்கு உதவும் கட்டளை வரி கருவியாகும். வகை, விருப்ப எல்லை, தலைப்பு, விவரம் மற்றும் அடிக்குறிப்பு ஆகியவற்றைக் கொண்ட கமிட் செய்தியை படிப்படியாக உருவாக்க ஊடாடும் இடைமுகத்தை இது வழங்குகிறது.

## சிறப்பம்சங்கள்

- **அடிப்படை செயல்பாடுகள்**
  - கமிட் வகைகள் மற்றும் எல்லைகளின் ஊடாடும் தேர்வு
  - `.commitkit.toml` மூலம் தனிப்பயனாக்கக்கூடிய அமைப்பு
  - தலைப்பின் அதிகபட்ச நீளத்தைக் கட்டுப்படுத்துதல்
  - பல வரி கமிட் விவரங்களுக்கான ஆதரவு
  - உள்ளீட்டின் தானியங்கி சரிபார்ப்பு
  - விருப்ப கையொப்பமிட்ட கமிட்கள் (`Signed-off-by`)
  - பல இயக்க முறைமைகளுக்கான இணக்கத்தன்மை

- **மேம்பட்ட அம்சங்கள்**
  - கமிட் செய்திகளில் எமோஜி ஆதரவு (✨, 🐛 போன்றவை)
  - இடநிரப்பி மாற்றீட்டுடன் கூடிய கமிட் வார்ப்புருக்கள்
  - தானியங்கி மாற்றப்பதிவு உருவாக்கம் மற்றும் புதுப்பிப்பு
  - கமிட் செய்தி சரிபார்ப்பு மற்றும் வடிவமைப்பிற்கான Git ஹூக்குகள்
  - கமிட் புள்ளிவிவரங்கள் மற்றும் பகுப்பாய்வு
  - வார்ப்புரு மதிப்புகளுக்கான ஊடாடும் கேள்விகள்

## முன் தேவைகள்

- **Rust**: பதிப்பு 1.88.0 அல்லது அதற்கு மேல் (MSRV)
- **Cargo**: Rust நிறுவலுடன் சேர்க்கப்பட்டுள்ளது

## நிறுவல்

### மூலக் குறியீட்டிலிருந்து

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## பயன்பாடு

உங்கள் Git களஞ்சியத்திற்குச் சென்று இயக்கவும்:

```shell
commitkit
```

### கட்டளை வரி விருப்பங்கள்

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit ஐத் தவிர்த்து செய்தியை stdout இல் அச்சிடு
    -v, --verbose           விரிவான வெளியீட்டைக் காட்டு
    -c, --config            அமைப்புக் கோப்பின் பாதை (இயல்புநிலை: நடப்பு அல்லது முகப்பு கோப்புறையில் .commitkit.toml)
        --emoji             கமிட் செய்திகளில் எமோஜியை இயக்கு அல்லது முடக்கு
        --template          குறிப்பிட்ட கமிட் வார்ப்புருவைப் பயன்படுத்து
        --changelog         கமிட் செய்த பிறகு மாற்றப்பதிவைப் புதுப்பி
        --install-hooks     git ஹூக்குகளை நிறுவு
        --stats             கமிட் புள்ளிவிவரங்களைக் காட்டு
        --days              புள்ளிவிவரங்களுக்கான பகுப்பாய்வு நாட்கள் (இயல்புநிலை: அனைத்தும்)
        --validate          கமிட் செய்திக் கோப்பைச் சரிபார்
        --prepare-msg       கமிட் செய்தியைத் தயாரி (git ஹூக்கால் பயன்படுத்தப்படும்)
        --lang <LANG>       காட்சி மொழி (எ.கா. en, ta, te, hi, ja)
    -h, --help              உதவித் தகவலை அச்சிடு
    -V, --version           பதிப்புத் தகவலை அச்சிடு
```

### ஆதரிக்கப்படும் மொழிகள்

CommitKit காட்சி மொழியை `LC_ALL`, `LC_MESSAGES` மற்றும் `LANG` ஆகியவற்றிலிருந்து கண்டறிகிறது அல்லது `--lang <LANG>` ஐ ஏற்கிறது. வலமிருந்து இடமாக எழுதப்படும் 6 மொழிகள் (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`) உட்பட 45 மொழிகளை இது ஆதரிக்கிறது. முழு பட்டியலுக்கு [ஆங்கில README](../../../README.md) ஐப் பார்க்கவும்.

### ஊடாடும் செயல்முறை

மரபுசார் கமிட் செய்தியை உருவாக்க இக்கருவி உங்களை வழிநடத்தும்:

1. கமிட் வகையைத் தேர்ந்தெடுக்கவும் (`feat`, `fix`, `docs` போன்றவை)
2. எல்லையைத் தேர்ந்தெடுக்கவும் (விருப்பத்திற்குரியது)
3. கமிட் தலைப்பை உள்ளிடவும்
4. கமிட் விவரத்தை உள்ளிடவும் (விருப்பத்திற்குரியது)
5. கமிட் அடிக்குறிப்பை உள்ளிடவும் (விருப்பத்திற்குரியது)

இந்தப் படிகளை முடித்த பிறகு, CommitKit உருவாக்கப்பட்ட செய்தியுடன் `git commit` ஐ இயக்கும்.

### எடுத்துக்காட்டுகள்

```shell
# அடிப்படை பயன்பாடு
commitkit

# சோதனை முறை - கமிட் செய்யாமல் உருவான செய்தியைக் காட்டு
commitkit --dry-run

# தனிப்பயன் அமைப்புக் கோப்பைப் பயன்படுத்து
commitkit --config /path/to/custom-config.toml

# விரிவான வெளியீடு
commitkit --verbose

# கமிட் செய்திகளில் எமோஜிகளைப் பயன்படுத்து
commitkit --emoji

# உங்கள் கமிட்டிற்கு வார்ப்புருவைப் பயன்படுத்து
commitkit --template feature

# CHANGELOG.md ஐ தானாகப் புதுப்பி
commitkit --changelog

# git ஹூக்குகளை நிறுவு
commitkit --install-hooks

# கடந்த 30 நாட்களுக்கான கமிட் புள்ளிவிவரங்களைக் காட்டு
commitkit --stats --days 30

# கமிட் செய்தியைச் சரிபார்
commitkit --validate .git/COMMIT_EDITMSG
```

## மேம்பாடு

### உருவாக்கம்

```bash
# பிழைதிருத்த உருவாக்கம் (Debug)
cargo build

# வெளியீட்டு உருவாக்கம் (Release)
cargo build --release
```

### சோதனை

```bash
# அனைத்து சோதனைகளையும் இயக்கு
cargo test

# வெளியீட்டுடன் சோதனைகளை இயக்கு
cargo test -- --nocapture
```

### குறியீட்டுச் சரிபார்ப்பு (Lint)

```bash
# Clippy எச்சரிக்கைகளைச் சரிபார்
cargo clippy --all-targets --all-features

# தானியங்கி திருத்தங்களைப் பயன்படுத்து
cargo clippy --fix --allow-dirty
```

### வடிவமைப்பு

```bash
# வடிவமைப்பைச் சரிபார்
cargo fmt -- --check

# வடிவமைப்பைப் பயன்படுத்து
cargo fmt
```

## அமைப்பு

CommitKit `.commitkit.toml` என்ற அமைப்புக் கோப்பை பின்வரும் இடங்களில் (வரிசையாக) தேடுகிறது:

1. நடப்பு கோப்புறை
2. பயனரின் முகப்பு கோப்புறை

எந்த அமைப்புக் கோப்பும் கிடைக்கவில்லை என்றால், உங்கள் முகப்பு கோப்புறையில் இயல்புநிலை கோப்பு உருவாக்கப்படும்.

அமைப்பு எடுத்துக்காட்டு:

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

## உரிமம்

இந்தத் திட்டம் MIT உரிமத்தின் கீழ் உரிமம் பெற்றுள்ளது — விவரங்களுக்கு [LICENSE](../../../LICENSE) கோப்பைப் பார்க்கவும்.

## பங்களிப்பு

பங்களிப்புகள் வரவேற்கப்படுகின்றன! தயங்காமல் issues உருவாக்குங்கள் அல்லது pull requests அனுப்புங்கள்.

1. களஞ்சியத்தை Fork செய்யவும்
2. உங்கள் அம்சக் கிளையை உருவாக்கவும் (`git checkout -b feature/amazing-feature`)
3. மரபுசார் கமிட் வடிவத்தில் உங்கள் மாற்றங்களை கமிட் செய்யவும்
4. கிளையை Push செய்யவும் (`git push origin feature/amazing-feature`)
5. ஒரு Pull Request திறக்கவும்
