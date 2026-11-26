[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit అనేది [Conventional Commits](https://www.conventionalcommits.org/) ప్రమాణాన్ని అనుసరించి క్రమబద్ధమైన మరియు స్థిరమైన కమిట్ సందేశాలను రూపొందించడంలో డెవలపర్లకు సహాయపడే కమాండ్-లైన్ సాధనం. రకం, ఐచ్ఛిక పరిధి, శీర్షిక, వివరాలు మరియు ఫుటర్‌తో కమిట్ సందేశాన్ని రూపొందించడానికి ఇది ఇంటరాక్టివ్ మార్గదర్శకత్వాన్ని అందిస్తుంది.

## లక్షణాలు

- **ప్రధాన కార్యాచరణ**
  - కమిట్ రకాలు మరియు పరిధుల ఇంటరాక్టివ్ ఎంపిక
  - `.commitkit.toml` ద్వారా అనుకూలీకరించదగిన కాన్ఫిగరేషన్
  - శీర్షిక గరిష్ఠ పొడవు నియంత్రణ
  - బహుళ-వరుస కమిట్ వివరాలకు మద్దతు
  - ఇన్‌పుట్ యొక్క స్వయంచాలక ధృవీకరణ
  - కమిట్లలో ఐచ్ఛిక `Signed-off-by` లైన్ జోడింపు
  - వివిధ ఆపరేటింగ్ సిస్టమ్‌లలో అనుకూలత

- **అధునాతన లక్షణాలు**
  - కమిట్ సందేశాలలో ఎమోజీ మద్దతు (✨, 🐛 మొదలైనవి)
  - ప్లేస్‌హోల్డర్ ప్రత్యామ్నాయంతో కమిట్ టెంప్లేట్‌లు
  - మార్పుల పట్టిక యొక్క స్వయంచాలక రూపకల్పన మరియు నవీకరణ
  - కమిట్ సందేశ ధృవీకరణ మరియు ఫార్మాటింగ్ కోసం Git హుక్స్
  - కమిట్ గణాంకాలు మరియు విశ్లేషణ
  - టెంప్లేట్ విలువల కోసం ఇంటరాక్టివ్ ప్రశ్నలు

## ముందస్తు అవసరాలు

- **Rust**: వెర్షన్ 1.88.0 లేదా అంతకంటే ఎక్కువ (MSRV)
- **Cargo**: Rust ఇన్‌స్టాలేషన్‌తో పాటు లభిస్తుంది

## ఇన్‌స్టాలేషన్

### సోర్స్ కోడ్ నుండి

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## వినియోగం

మీ Git రిపోజిటరీలోకి వెళ్లి ఈ క్రింది కమాండ్‌ను అమలు చేయండి:

```shell
commitkit
```

### కమాండ్-లైన్ ఎంపికలు

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit ను దాటవేసి సందేశాన్ని stdout కు ముద్రించండి
    -v, --verbose           వివరమైన అవుట్‌పుట్‌ను చూపించు
    -c, --config            కాన్ఫిగ్ ఫైల్ మార్గం (డిఫాల్ట్: ప్రస్తుత లేదా హోమ్ డైరెక్టరీలో .commitkit.toml)
        --emoji             కమిట్ సందేశాలలో ఎమోజీలను ఆన్ లేదా ఆఫ్ చేయండి
        --template          నిర్దిష్ట కమిట్ టెంప్లేట్‌ను ఉపయోగించండి
        --changelog         కమిట్ తర్వాత మార్పుల పట్టికను నవీకరించండి
        --install-hooks     git హుక్స్‌ను ఇన్‌స్టాల్ చేయండి
        --stats             కమిట్ గణాంకాలను చూపించు
        --days              గణాంకాల విశ్లేషణ కోసం రోజుల సంఖ్య (డిఫాల్ట్: అన్నీ)
        --validate          కమిట్ సందేశ ఫైల్‌ను ధృవీకరించండి
        --prepare-msg       కమిట్ సందేశాన్ని సిద్ధం చేయండి (git హుక్ ఉపయోగిస్తుంది)
        --lang <LANG>       ప్రదర్శన భాష (ఉదా. en, te, ta, hi, ja)
    -h, --help              సహాయ సమాచారాన్ని ముద్రించండి
    -V, --version           వెర్షన్ సమాచారాన్ని ముద్రించండి
```

### మద్దతు ఉన్న భాషలు

CommitKit ప్రదర్శన భాషను `LC_ALL`, `LC_MESSAGES` మరియు `LANG` నుండి గుర్తిస్తుంది లేదా `--lang <LANG>` ను అంగీకరిస్తుంది. ఇది కుడి-నుండి-ఎడమకు రాసే 6 భాషలు (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`) సహా 45 భాషలకు మద్దతు ఇస్తుంది. పూర్తి జాబితా కోసం [ఆంగ్ల README](../../../README.md) చూడండి.

### ఇంటరాక్టివ్ ప్రక్రియ

సంప్రదాయ కమిట్ సందేశాన్ని రూపొందించడంలో ఈ సాధనం మీకు మార్గనిర్దేశం చేస్తుంది:

1. కమిట్ రకాన్ని ఎంచుకోండి (`feat`, `fix`, `docs` మొదలైనవి)
2. పరిధిని ఎంచుకోండి (ఐచ్ఛికం)
3. కమిట్ శీర్షికను నమోదు చేయండి
4. కమిట్ వివరాలను నమోదు చేయండి (ఐచ్ఛికం)
5. కమిట్ ఫుటర్‌ను నమోదు చేయండి (ఐచ్ఛికం)

ఈ దశలు పూర్తయిన తర్వాత CommitKit మీరు రూపొందించిన సందేశంతో `git commit` ను అమలు చేస్తుంది.

### ఉదాహరణలు

```shell
# ప్రాథమిక వినియోగం
commitkit

# పరీక్షా మోడ్ - కమిట్ చేయకుండా రూపొందించిన సందేశాన్ని చూపించు
commitkit --dry-run

# అనుకూల కాన్ఫిగ్ ఫైల్‌ను ఉపయోగించండి
commitkit --config /path/to/custom-config.toml

# వివరమైన అవుట్‌పుట్
commitkit --verbose

# కమిట్ సందేశాలలో ఎమోజీలను ఉపయోగించండి
commitkit --emoji

# మీ కమిట్ కోసం టెంప్లేట్ ఉపయోగించండి
commitkit --template feature

# CHANGELOG.md ను స్వయంచాలకంగా నవీకరించండి
commitkit --changelog

# git హుక్స్‌ను ఇన్‌స్టాల్ చేయండి
commitkit --install-hooks

# గత 30 రోజుల కమిట్ గణాంకాలను చూపించు
commitkit --stats --days 30

# కమిట్ సందేశాన్ని ధృవీకరించండి
commitkit --validate .git/COMMIT_EDITMSG
```

## అభివృద్ధి

### బిల్డ్

```bash
# డీబగ్ బిల్డ్
cargo build

# రిలీజ్ బిల్డ్
cargo build --release
```

### పరీక్షలు

```bash
# అన్ని పరీక్షలను అమలు చేయండి
cargo test

# అవుట్‌పుట్‌తో పరీక్షలను అమలు చేయండి
cargo test -- --nocapture
```

### లింట్

```bash
# Clippy హెచ్చరికలను తనిఖీ చేయండి
cargo clippy --all-targets --all-features

# స్వయంచాలక దిద్దుబాట్లను వర్తింపజేయండి
cargo clippy --fix --allow-dirty
```

### ఫార్మాటింగ్

```bash
# ఫార్మాటింగ్‌ను తనిఖీ చేయండి
cargo fmt -- --check

# ఫార్మాటింగ్‌ను వర్తింపజేయండి
cargo fmt
```

## కాన్ఫిగరేషన్

CommitKit ఈ క్రింది స్థానాలలో (క్రమంగా) `.commitkit.toml` అనే కాన్ఫిగరేషన్ ఫైల్ కోసం వెతుకుతుంది:

1. ప్రస్తుత డైరెక్టరీ
2. వినియోగదారు యొక్క హోమ్ డైరెక్టరీ

కాన్ఫిగరేషన్ ఫైల్ కనుగొనబడకపోతే, మీ హోమ్ డైరెక్టరీలో డిఫాల్ట్ ఫైల్ రూపొందించబడుతుంది.

ఉదాహరణ కాన్ఫిగరేషన్:

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

## లైసెన్స్

ఈ ప్రాజెక్ట్ MIT లైసెన్స్ కింద లైసెన్స్ పొందింది — వివరాల కోసం [LICENSE](../../../LICENSE) ఫైల్‌ను చూడండి.

## సహకారం

సహకారాలు ఎల్లప్పుడూ స్వాగతించబడతాయి! నిరభ్యంతరంగా issues తెరవండి లేదా pull requests సమర్పించండి.

1. రిపోజిటరీని Fork చేయండి
2. మీ ఫీచర్ బ్రాంచ్‌ను సృష్టించండి (`git checkout -b feature/amazing-feature`)
3. సంప్రదాయ కమిట్‌లను ఉపయోగించి మీ మార్పులను కమిట్ చేయండి
4. బ్రాంచ్‌ను Push చేయండి (`git push origin feature/amazing-feature`)
5. ఒక Pull Request తెరవండి
