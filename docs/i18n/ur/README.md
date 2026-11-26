[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit ایک کمانڈ لائن ٹول ہے جو ڈویلپرز کو [Conventional Commits](https://www.conventionalcommits.org/) معیار کے مطابق منظم اور یکساں کمٹ پیغامات تیار کرنے میں مدد دیتا ہے۔ یہ ایک انٹرایکٹو انٹرفیس فراہم کرتا ہے جو قسم، اختیاری دائرہ کار، عنوان، متن اور فوٹر کے ساتھ کمٹ پیغام ترتیب دینے میں قدم بہ قدم رہنمائی کرتا ہے۔

## خصوصیات

- **بنیادی فعالیت**
  - کمٹ اقسام اور دائرہ کار کا انٹرایکٹو انتخاب
  - `.commitkit.toml` کے ذریعے قابلِ ترمیم کنفیگریشن
  - عنوان کی زیادہ سے زیادہ لمبائی کی پابندی
  - کثیر سطری کمٹ متن کی سہولت
  - ان پٹ کی خودکار توثیق
  - کمٹس میں اختیاری `Signed-off-by` سطر شامل کرنے کی سہولت
  - مختلف آپریٹنگ سسٹمز کے ساتھ مطابقت

- **اضافی خصوصیات**
  - کمٹ پیغامات میں ایموجی کی سہولت (✨، 🐛 وغیرہ)
  - متغیرات کے متبادل کے ساتھ کمٹ ٹیمپلیٹس
  - تبدیلیوں کے ریکارڈ کی خودکار تیاری اور اپ ڈیٹ
  - کمٹ پیغامات کی توثیق اور فارمیٹنگ کے لیے Git ہکس
  - کمٹ کے اعداد و شمار اور تجزیہ
  - ٹیمپلیٹ اقدار کے لیے انٹرایکٹو سوالات

## پیشگی تقاضے

- **Rust**: ورژن 1.88.0 یا اس سے جدید (MSRV)
- **Cargo**: Rust کی تنصیب کے ساتھ شامل ہے

## تنصیب

### سورس کوڈ سے

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## استعمال

اپنی Git ریپوزٹری میں جائیں اور یہ کمانڈ چلائیں:

```shell
commitkit
```

### کمانڈ لائن اختیارات

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit چھوڑ کر پیغام stdout پر پرنٹ کریں
    -v, --verbose           تفصیلی آؤٹ پٹ دکھائیں
    -c, --config            کنفگ فائل کا راستہ (طے شدہ: موجودہ یا ہوم ڈائریکٹری میں .commitkit.toml)
        --emoji             کمٹ پیغامات میں ایموجی آن یا آف کریں
        --template          مخصوص کمٹ ٹیمپلیٹ استعمال کریں
        --changelog         کمٹ کے بعد تبدیلیوں کا ریکارڈ اپ ڈیٹ کریں
        --install-hooks     git ہکس انسٹال کریں
        --stats             کمٹ کے اعداد و شمار دکھائیں
        --days              اعداد و شمار کے تجزیے کے لیے دنوں کی تعداد (طے شدہ: سب)
        --validate          کمٹ پیغام کی فائل کی توثیق کریں
        --prepare-msg       کمٹ پیغام تیار کریں (git ہک کے ذریعے استعمال ہوتا ہے)
        --lang <LANG>       ڈسپلے زبان (مثلاً en, ur, hi, ar, fa)
    -h, --help              مدد کی معلومات پرنٹ کریں
    -V, --version           ورژن کی معلومات پرنٹ کریں
```

### معاون زبانیں

CommitKit ڈسپلے زبان کی شناخت `LC_ALL`، `LC_MESSAGES` اور `LANG` سے کرتا ہے یا `--lang <LANG>` قبول کرتا ہے۔ یہ 45 زبانوں کی حمایت کرتا ہے جن میں دائیں سے بائیں لکھی جانے والی 6 زبانیں (`ar`، `ar-EG`، `ar-MA`، `fa`، `he` اور `ur`) شامل ہیں۔ مکمل جدول کے لیے [انگریزی README](../../../README.md) دیکھیں۔

### انٹرایکٹو عمل

یہ ٹول معیاری کمٹ پیغام بنانے کے مراحل میں آپ کی رہنمائی کرے گا:

1. کمٹ کی قسم منتخب کریں (`feat`، `fix`، `docs` وغیرہ)
2. دائرہ کار منتخب کریں (اختیاری)
3. کمٹ کا عنوان درج کریں
4. کمٹ کا متن درج کریں (اختیاری)
5. کمٹ کا فوٹر درج کریں (اختیاری)

ان مراحل کے مکمل ہونے کے بعد CommitKit آپ کے تیار کردہ پیغام کے ساتھ `git commit` چلائے گا۔

### مثالیں

```shell
# بنیادی استعمال
commitkit

# آزمائشی موڈ - کمٹ کیے بغیر تیار شدہ پیغام دکھائیں
commitkit --dry-run

# حسبِ ضرورت کنفگ فائل استعمال کریں
commitkit --config /path/to/custom-config.toml

# تفصیلی آؤٹ پٹ
commitkit --verbose

# کمٹ پیغامات میں ایموجی استعمال کریں
commitkit --emoji

# اپنے کمٹ کے لیے ٹیمپلیٹ استعمال کریں
commitkit --template feature

# CHANGELOG.md کو خودکار طور پر اپ ڈیٹ کریں
commitkit --changelog

# git ہکس انسٹال کریں
commitkit --install-hooks

# گزشتہ 30 دنوں کے کمٹ اعداد و شمار دکھائیں
commitkit --stats --days 30

# کمٹ پیغام کی توثیق کریں
commitkit --validate .git/COMMIT_EDITMSG
```

## ڈیولپمنٹ

### بلڈ

```bash
# ڈیبگ بلڈ
cargo build

# ریلیز بلڈ
cargo build --release
```

### ٹیسٹ

```bash
# تمام ٹیسٹ چلائیں
cargo test

# آؤٹ پٹ کے ساتھ ٹیسٹ چلائیں
cargo test -- --nocapture
```

### لنٹ

```bash
# Clippy انتباہات چیک کریں
cargo clippy --all-targets --all-features

# خودکار اصلاحات لاگو کریں
cargo clippy --fix --allow-dirty
```

### فارمیٹنگ

```bash
# فارمیٹنگ چیک کریں
cargo fmt -- --check

# فارمیٹنگ لاگو کریں
cargo fmt
```

## کنفیگریشن

CommitKit درج ذیل مقامات پر (ترتیب وار) `.commitkit.toml` نامی کنفیگریشن فائل تلاش کرتا ہے:

1. موجودہ ڈائریکٹری
2. صارف کی ہوم ڈائریکٹری

اگر کوئی کنفیگریشن فائل نہ ملے تو آپ کی ہوم ڈائریکٹری میں ایک طے شدہ فائل خود بخود بنا دی جائے گی۔

مثالی کنفیگریشن:

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

## لائسنس

یہ پروجیکٹ MIT لائسنس کے تحت جاری کیا گیا ہے — تفصیلات کے لیے [LICENSE](../../../LICENSE) فائل دیکھیں۔

## تعاون

تعاون کا خیرمقدم ہے! بلا جھجک issues کھولیں یا pull requests جمع کرائیں۔

1. ریپوزٹری کو Fork کریں
2. اپنی فیچر برانچ بنائیں (`git checkout -b feature/amazing-feature`)
3. معیاری کمٹس استعمال کرتے ہوئے اپنی تبدیلیاں کمٹ کریں
4. برانچ کو Push کریں (`git push origin feature/amazing-feature`)
5. ایک Pull Request کھولیں
