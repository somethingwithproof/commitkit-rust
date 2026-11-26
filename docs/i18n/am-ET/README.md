[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit ገንቢዎች የ[Conventional Commits](https://www.conventionalcommits.org/) መደበኛ መስፈርትን የተከተሉ የተዋቀሩ እና ወጥነት ያላቸው የኮሚት መልዕክቶችን እንዲፈጥሩ የሚያግዝ የትዕዛዝ መስመር መሳሪያ ነው። ዓይነት፣ አማራጭ ወሰን፣ ርዕስ፣ አካል እና ግርጌ ያለው የኮሚት መልዕክት ደረጃ በደረጃ ለመገንባት በይነተገናኝ መመሪያ ይሰጣል።

## ባህሪያት

- **ዋና ተግባራት**
  - የኮሚት ዓይነቶችን እና ወሰኖችን በይነተገናኝ መምረጥ
  - በ`.commitkit.toml` አማካኝነት የሚስተካከል ውቅር
  - የርዕስ ከፍተኛ ርዝመት ቁጥጥር
  - ባለብዙ መስመር የኮሚት አካል ድጋፍ
  - የግብዓት ራስ-ሰር ማረጋገጫ
  - በኮሚቶች ውስጥ አማራጭ `Signed-off-by` መስመር
  - ለተለያዩ ስርዓተ ክወናዎች ተስማሚነት

- **የተሻሻሉ ባህሪያት**
  - በኮሚት መልዕክቶች ውስጥ የኢሞጂ ድጋፍ (✨፣ 🐛 ወዘተ)
  - ቦታ ያዥ የሚተኩ የኮሚት አብነቶች
  - ራስ-ሰር የለውጥ መዝገብ ማመንጨት እና ማዘመን
  - የኮሚት መልዕክት ማረጋገጫ እና አቀራረብ የሚያከናውኑ Git ሁኮች
  - የኮሚት ስታቲስቲክስ እና ትንተና
  - ለአብነት እሴቶች በይነተገናኝ ጥያቄዎች

## ቅድመ ሁኔታዎች

- **Rust**፡ ስሪት 1.88.0 ወይም ከዚያ በላይ (MSRV)
- **Cargo**፡ ከRust ጭነት ጋር የተካተተ

## መጫን

### ከምንጭ ኮድ

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## አጠቃቀም

ወደ Git ማከማቻዎ ይሂዱ እና ይህን ያስኪዱ፡

```shell
commitkit
```

### የትዕዛዝ መስመር አማራጮች

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit ሳይፈጽሙ መልዕክቱን ወደ stdout ያትሙ
    -v, --verbose           ዝርዝር ውጤት አሳይ
    -c, --config            የውቅር ፋይል መንገድ (ነባሪ፡ በአሁኑ ወይም በመነሻ ማውጫ ውስጥ ያለው .commitkit.toml)
        --emoji             በኮሚት መልዕክቶች ውስጥ ኢሞጂ አብራ ወይም አጥፋ
        --template          የተወሰነ የኮሚት ቅንብር አብነት ተጠቀም
        --changelog         ከኮሚት በኋላ የለውጥ መዝገብ አዘምን
        --install-hooks     የgit ሁኮችን ጫን
        --stats             የኮሚት ስታቲስቲክስ አሳይ
        --days              ለስታቲስቲክስ የሚተነተኑ ቀናት ብዛት (ነባሪ፡ ሁሉም)
        --validate          የኮሚት መልዕክት ፋይል ትክክለኛነት አረጋግጥ
        --prepare-msg       የኮሚት መልዕክት አዘጋጅ (በgit ሁክ የሚጠቀም)
        --lang <LANG>       የማሳያ ቋንቋ (ለምሳሌ en, am-ET, sw-KE, ar, ja)
    -h, --help              የእገዛ መረጃ አትም
    -V, --version           የስሪት መረጃ አትም
```

### የሚደገፉ ቋንቋዎች

CommitKit የማሳያ ቋንቋውን ከ`LC_ALL`፣ `LC_MESSAGES` እና `LANG` በራስ-ሰር ይለያል ወይም `--lang <LANG>` ይቀበላል። ከቀኝ ወደ ግራ የሚጻፉ 6 ቋንቋዎችን (`ar`፣ `ar-EG`፣ `ar-MA`፣ `fa`፣ `he`፣ `ur`) ጨምሮ 45 ቋንቋዎችን ይደግፋል። ሙሉውን ዝርዝር በ[እንግሊዝኛ README](../../../README.md) ውስጥ ይመልከቱ።

### በይነተገናኝ ሂደት

መሳሪያው መደበኛ የኮሚት መልዕክት ለመፍጠር ደረጃ በደረጃ ይመራዎታል፡

1. የኮሚት ዓይነት ይምረጡ (`feat`፣ `fix`፣ `docs` ወዘተ)
2. ወሰን ይምረጡ (አማራጭ)
3. የኮሚት ርዕስ ያስገቡ
4. የኮሚት አካል ያስገቡ (አማራጭ)
5. የኮሚት ግርጌ ያስገቡ (አማራጭ)

እነዚህን ደረጃዎች ከጨረሱ በኋላ CommitKit በተዘጋጀው መልዕክት `git commit` ያስፈጽማል።

### ምሳሌዎች

```shell
# መሰረታዊ አጠቃቀም
commitkit

# የሙከራ ሂደት - ኮሚት ሳይፈጸም የተፈጠረውን መልዕክት አሳይ
commitkit --dry-run

# ብጁ የውቅር ፋይል ተጠቀም
commitkit --config /path/to/custom-config.toml

# ዝርዝር ውጤት
commitkit --verbose

# በኮሚት መልዕክቶች ውስጥ ኢሞጂ ተጠቀም
commitkit --emoji

# ለኮሚትዎ አብነት ተጠቀም
commitkit --template feature

# CHANGELOG.md ን በራስ-ሰር አዘምን
commitkit --changelog

# የgit ሁኮችን ጫን
commitkit --install-hooks

# ላለፉት 30 ቀናት የኮሚት ስታቲስቲክስ አሳይ
commitkit --stats --days 30

# የኮሚት መልዕክትን አረጋግጥ
commitkit --validate .git/COMMIT_EDITMSG
```

## ልማት

### ግንባታ

```bash
# የማረሚያ ግንባታ
cargo build

# የመልቀቂያ ግንባታ
cargo build --release
```

### ሙከራ

```bash
# ሁሉንም ሙከራዎች አስኪድ
cargo test

# ውጤትን እያሳዩ ሙከራዎችን አስኪድ
cargo test -- --nocapture
```

### ሊንት

```bash
# የኮድ ጥራት ማስጠንቀቂያዎችን ፈትሽ
cargo clippy --all-targets --all-features

# ራስ-ሰር ማስተካከያዎችን ተግብር
cargo clippy --fix --allow-dirty
```

### ቅርጸት

```bash
# ቅርጸቱን አረጋግጥ
cargo fmt -- --check

# ቅርጸቱን ተግብር
cargo fmt
```

## ውቅር

CommitKit `.commitkit.toml` የተባለ የውቅር ፋይል በሚከተሉት ቦታዎች (በቅደም ተከተል) ይፈልጋል፡

1. አሁን ያሉበት ማውጫ
2. የተጠቃሚው መነሻ ማውጫ

ምንም የውቅር ፋይል ካልተገኘ በመነሻ ማውጫዎ ውስጥ ነባሪ ፋይል ይፈጠራል።

የውቅር ምሳሌ፡

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

## ፈቃድ

ይህ ፕሮጀክት በMIT ፈቃድ ስር ተሰራጭቷል — ለዝርዝሮች የ[LICENSE](../../../LICENSE) ፋይልን ይመልከቱ።

## አስተዋጽኦ

አስተዋጽኦዎች ሁልጊዜ ይቀበላሉ! ችግሮችን (issues) ለመክፈት ወይም pull requests ለመላክ ነጻነት ይሰማዎ።

1. ማከማቻውን ፎርክ ያድርጉ
2. የባህሪ ቅርንጫፍዎን ይፍጠሩ (`git checkout -b feature/amazing-feature`)
3. ለውጦችዎን በመደበኛ የኮሚት ቅርጸት ኮሚት ያድርጉ
4. ቅርንጫፉን ይግፉ (`git push origin feature/amazing-feature`)
5. Pull Request ይክፈቱ
