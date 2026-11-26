[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit एक कमांड-लाइन टूल है जो डेवलपर्स को [Conventional Commits](https://www.conventionalcommits.org/) मानक के अनुरूप सुव्यवस्थित और एकसमान कमिट संदेश तैयार करने में मदद करता है। यह एक संवादात्मक इंटरफ़ेस प्रदान करता है जो प्रकार, वैकल्पिक दायरे, विषय, विवरण और फ़ुटर के साथ कमिट संदेश बनाने में आपका मार्गदर्शन करता है।

## विशेषताएँ

- **मुख्य कार्यक्षमता**
  - कमिट प्रकार और दायरे का संवादात्मक चयन
  - `.commitkit.toml` के माध्यम से अनुकूलन योग्य कॉन्फ़िगरेशन
  - विषय की अधिकतम लंबाई का पालन
  - बहु-पंक्ति कमिट विवरण का समर्थन
  - इनपुट की स्वतः पुष्टि
  - वैकल्पिक हस्ताक्षरित कमिट (`Signed-off-by`)
  - क्रॉस-प्लेटफ़ॉर्म संगतता

- **उन्नत विशेषताएँ**
  - कमिट संदेशों में इमोजी का समर्थन (✨, 🐛 आदि)
  - प्लेसहोल्डर प्रतिस्थापन वाले कमिट टेम्पलेट
  - चेंजलॉग का स्वतः निर्माण और अद्यतन
  - कमिट संदेश सत्यापन और फ़ॉर्मेटिंग के लिए Git हुक
  - कमिट सांख्यिकी और विश्लेषण
  - टेम्पलेट मानों के लिए संवादात्मक संकेत

## पूर्वापेक्षाएँ

- **Rust**: संस्करण 1.88.0 या उच्चतर (MSRV)
- **Cargo**: Rust इंस्टॉलेशन के साथ शामिल

## स्थापना

### स्रोत से

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## उपयोग

अपनी Git रिपॉजिटरी में जाएँ और चलाएँ:

```shell
commitkit
```

### कमांड-लाइन विकल्प

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit को छोड़कर संदेश को stdout पर प्रिंट करें
    -v, --verbose           विस्तृत आउटपुट दिखाएँ
    -c, --config            कॉन्फ़िग फ़ाइल का पथ (डिफ़ॉल्ट: वर्तमान या होम निर्देशिका में .commitkit.toml)
        --emoji             कमिट संदेशों में इमोजी टॉगल करें
        --template          किसी विशिष्ट कमिट टेम्पलेट का उपयोग करें
        --changelog         कमिट करने के बाद चेंजलॉग अद्यतन करें
        --install-hooks     git हुक स्थापित करें
        --stats             कमिट सांख्यिकी दिखाएँ
        --days              सांख्यिकी के विश्लेषण के लिए दिनों की संख्या (डिफ़ॉल्ट: सभी)
        --validate          कमिट संदेश फ़ाइल को मान्य करें
        --prepare-msg       कमिट संदेश तैयार करें (git हुक द्वारा उपयोग किया जाता है)
        --lang <LANG>       प्रदर्शन भाषा (जैसे en, de, ja, zh-CN, pt-BR)
    -h, --help              सहायता जानकारी प्रिंट करें
    -V, --version           संस्करण जानकारी प्रिंट करें
```

### समर्थित भाषाएँ

CommitKit `LC_ALL`, `LC_MESSAGES` और `LANG` से प्रदर्शन भाषा का स्वतः पता लगाता है या `--lang <LANG>` स्वीकार करता है। यह 45 भाषाओं का समर्थन करता है, जिनमें दाएँ-से-बाएँ (RTL) लिखी जाने वाली 6 भाषाएँ (`ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur`) शामिल हैं। पूरी तालिका के लिए [अंग्रेज़ी README](../../../README.md) देखें।

### संवादात्मक प्रक्रिया

यह टूल पारंपरिक कमिट संदेश बनाने के चरणों में आपका मार्गदर्शन करेगा:

1. कमिट प्रकार चुनें (`feat`, `fix`, `docs` आदि)
2. दायरा चुनें (वैकल्पिक)
3. कमिट विषय दर्ज करें
4. कमिट विवरण दर्ज करें (वैकल्पिक)
5. कमिट फ़ुटर दर्ज करें (वैकल्पिक)

इन चरणों के पूरा होने पर, CommitKit आपके तैयार किए गए संदेश के साथ `git commit` चलाएगा।

### उदाहरण

```shell
# बुनियादी उपयोग
commitkit

# ड्राई रन - बिना कमिट किए परिणामी कमिट संदेश दिखाएँ
commitkit --dry-run

# कस्टम कॉन्फ़िग फ़ाइल का उपयोग करें
commitkit --config /path/to/custom-config.toml

# विस्तृत आउटपुट
commitkit --verbose

# कमिट संदेशों में इमोजी का उपयोग करें
commitkit --emoji

# अपने कमिट के लिए टेम्पलेट का उपयोग करें
commitkit --template feature

# CHANGELOG.md को स्वतः अद्यतन करें
commitkit --changelog

# git हुक स्थापित करें
commitkit --install-hooks

# पिछले 30 दिनों की कमिट सांख्यिकी दिखाएँ
commitkit --stats --days 30

# किसी कमिट संदेश को मान्य करें
commitkit --validate .git/COMMIT_EDITMSG
```

## विकास

### बिल्ड

```bash
# डीबग बिल्ड
cargo build

# रिलीज़ बिल्ड
cargo build --release
```

### परीक्षण

```bash
# सभी परीक्षण चलाएँ
cargo test

# आउटपुट के साथ परीक्षण चलाएँ
cargo test -- --nocapture
```

### लिंट

```bash
# लिंटिंग समस्याओं की जाँच करें
cargo clippy --all-targets --all-features

# स्वचालित सुधार लागू करें
cargo clippy --fix --allow-dirty
```

### फ़ॉर्मेटिंग

```bash
# फ़ॉर्मेटिंग जाँचें
cargo fmt -- --check

# फ़ॉर्मेटिंग लागू करें
cargo fmt
```

## कॉन्फ़िगरेशन

CommitKit निम्नलिखित स्थानों में (क्रम से) `.commitkit.toml` नामक कॉन्फ़िगरेशन फ़ाइल खोजता है:

1. वर्तमान निर्देशिका
2. उपयोगकर्ता की होम निर्देशिका

यदि कोई कॉन्फ़िगरेशन फ़ाइल नहीं मिलती है, तो आपकी होम निर्देशिका में एक डिफ़ॉल्ट फ़ाइल तैयार कर दी जाएगी।

उदाहरण कॉन्फ़िगरेशन:

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

## लाइसेंस

यह प्रोजेक्ट MIT लाइसेंस के अंतर्गत लाइसेंस प्राप्त है — विवरण के लिए [LICENSE](../../../LICENSE) फ़ाइल देखें।

## योगदान

योगदान का स्वागत है! बेझिझक इश्यू खोलें या पुल रिक्वेस्ट भेजें।

1. रिपॉजिटरी को फ़ॉर्क करें
2. अपनी फ़ीचर ब्रांच बनाएँ (`git checkout -b feature/amazing-feature`)
3. पारंपरिक कमिट्स का उपयोग करके अपने बदलाव कमिट करें
4. ब्रांच पुश करें (`git push origin feature/amazing-feature`)
5. एक Pull Request खोलें
