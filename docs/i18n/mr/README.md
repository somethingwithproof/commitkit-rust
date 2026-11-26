[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit हे एक कमांड-लाइन साधन आहे जे विकासकांना [Conventional Commits](https://www.conventionalcommits.org/) मानकानुसार सुव्यवस्थित आणि सुसंगत कमिट संदेश तयार करण्यात मदत करते. हे साधन प्रकार, पर्यायी व्याप्ती, विषय, मुख्य मजकूर आणि फूटरसह कमिट संदेश तयार करण्यासाठी संवादात्मक मार्गदर्शन प्रदान करते.

## वैशिष्ट्ये

- **मुख्य कार्यक्षमता**
  - कमिट प्रकार आणि व्याप्तीची संवादात्मक निवड
  - `.commitkit.toml` द्वारे सानुकूल करता येणारे कॉन्फिगरेशन
  - विषयाच्या कमाल लांबीचे नियंत्रण
  - बहु-ओळींच्या कमिट मजकुरासाठी समर्थन
  - इनपुटची स्वयंचलित तपासणी
  - पर्यायी स्वाक्षरीकृत कमिट (`Signed-off-by`)
  - विविध ऑपरेटिंग सिस्टम्सशी सुसंगतता

- **प्रगत वैशिष्ट्ये**
  - कमिट संदेशांमध्ये इमोजी समर्थन (✨, 🐛 इ.)
  - प्लेसहोल्डर बदलासह कमिट टेम्पलेट्स
  - बदलनोंदीची स्वयंचलित निर्मिती आणि अद्यतन
  - कमिट संदेश प्रमाणित आणि फॉरमॅट करण्यासाठी Git हुक
  - कमिट आकडेवारी आणि विश्लेषण
  - टेम्पलेट मूल्यांसाठी संवादात्मक सूचना

## पूर्वआवश्यकता

- **Rust**: आवृत्ती 1.88.0 किंवा त्यापुढील (MSRV)
- **Cargo**: Rust स्थापनेसह समाविष्ट

## स्थापना

### स्रोत कोडवरून

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## वापर

तुमच्या Git रिपॉझिटरीमध्ये जा आणि खालील कमांड चालवा:

```shell
commitkit
```

### कमांड-लाइन पर्याय

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit वगळा आणि संदेश stdout वर छापा
    -v, --verbose           सविस्तर आउटपुट दाखवा
    -c, --config            कॉन्फिग फाइलचा मार्ग (डीफॉल्ट: सध्याच्या किंवा होम डिरेक्टरीमधील .commitkit.toml)
        --emoji             कमिट संदेशांमध्ये इमोजी सुरू किंवा बंद करा
        --template          विशिष्ट कमिट टेम्पलेट वापरा
        --changelog         कमिट केल्यानंतर बदलनोंद अद्यतनित करा
        --install-hooks     git हुक स्थापित करा
        --stats             कमिट आकडेवारी दाखवा
        --days              आकडेवारीच्या विश्लेषणासाठी दिवसांची संख्या (डीफॉल्ट: सर्व)
        --validate          कमिट संदेश फाइल तपासा
        --prepare-msg       कमिट संदेश तयार करा (git हुकद्वारे वापरला जातो)
        --lang <LANG>       प्रदर्शन भाषा (उदा. en, mr, hi, bn, ta)
    -h, --help              मदत माहिती छापा
    -V, --version           आवृत्ती माहिती छापा
```

### समर्थित भाषा

CommitKit `LC_ALL`, `LC_MESSAGES` आणि `LANG` वरून प्रदर्शन भाषा ओळखते किंवा `--lang <LANG>` स्वीकारते. हे ४५ भाषांना समर्थन देते, ज्यात उजवीकडून डावीकडे लिहिल्या जाणाऱ्या ६ भाषा (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`) समाविष्ट आहेत. संपूर्ण यादीसाठी [इंग्रजी README](../../../README.md) पहा.

### संवादात्मक प्रक्रिया

हे साधन प्रमाणित कमिट संदेश तयार करण्यासाठी तुम्हाला पायरीपायरीने मार्गदर्शन करेल:

1. कमिट प्रकार निवडा (`feat`, `fix`, `docs` इ.)
2. व्याप्ती निवडा (पर्यायी)
3. कमिट विषय प्रविष्ट करा
4. कमिटचा मुख्य मजकूर प्रविष्ट करा (पर्यायी)
5. कमिट फूटर प्रविष्ट करा (पर्यायी)

या पायऱ्या पूर्ण झाल्यावर CommitKit तयार केलेल्या संदेशासह `git commit` कार्यान्वित करेल.

### उदाहरणे

```shell
# मूलभूत वापर
commitkit

# चाचणी मोड - कमिट न करता तयार होणारा संदेश दाखवा
commitkit --dry-run

# सानुकूल कॉन्फिग फाइल वापरा
commitkit --config /path/to/custom-config.toml

# सविस्तर आउटपुट
commitkit --verbose

# कमिट संदेशांमध्ये इमोजी वापरा
commitkit --emoji

# तुमच्या कमिटसाठी टेम्पलेट वापरा
commitkit --template feature

# CHANGELOG.md स्वयंचलितपणे अद्यतनित करा
commitkit --changelog

# git हुक स्थापित करा
commitkit --install-hooks

# मागील ३० दिवसांची कमिट आकडेवारी दाखवा
commitkit --stats --days 30

# कमिट संदेश तपासा
commitkit --validate .git/COMMIT_EDITMSG
```

## विकास

### बिल्ड

```bash
# डीबग बिल्ड
cargo build

# रिलीज बिल्ड
cargo build --release
```

### चाचणी

```bash
# सर्व चाचण्या चालवा
cargo test

# आउटपुटसह चाचण्या चालवा
cargo test -- --nocapture
```

### लिंट

```bash
# Clippy चेतावण्या तपासा
cargo clippy --all-targets --all-features

# स्वयंचलित दुरुस्त्या लागू करा
cargo clippy --fix --allow-dirty
```

### फॉरमॅटिंग

```bash
# फॉरमॅटिंग तपासा
cargo fmt -- --check

# फॉरमॅटिंग लागू करा
cargo fmt
```

## कॉन्फिगरेशन

CommitKit खालील ठिकाणी (क्रमाने) `.commitkit.toml` नावाची कॉन्फिगरेशन फाइल शोधते:

1. सध्याची डिरेक्टरी
2. वापरकर्त्याची होम डिरेक्टरी

कोणतीही कॉन्फिगरेशन फाइल न सापडल्यास तुमच्या होम डिरेक्टरीमध्ये डीफॉल्ट फाइल तयार केली जाईल.

उदाहरण कॉन्फिगरेशन:

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

## परवाना

हा प्रकल्प MIT परवान्याअंतर्गत वितरित केला आहे — तपशीलासाठी [LICENSE](../../../LICENSE) फाइल पहा.

## योगदान

योगदानाचे स्वागत आहे! कृपया issue उघडा किंवा pull request पाठवा.

1. रिपॉझिटरी Fork करा
2. तुमची फीचर शाखा तयार करा (`git checkout -b feature/amazing-feature`)
3. प्रमाणित कमिट संदेश वापरून बदल कमिट करा
4. शाखा पुश करा (`git push origin feature/amazing-feature`)
5. Pull Request उघडा
