[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit একটি কমান্ড-লাইন টুল যা ডেভেলপারদের [Conventional Commits](https://www.conventionalcommits.org/) নীতিমালা অনুসরণ করে সুগঠিত এবং সামঞ্জস্যপূর্ণ কমিট বার্তা তৈরি করতে সহায়তা করে। এটি একটি ইন্টারেক্টিভ ইন্টারফেস প্রদান করে যা ধরন, ঐচ্ছিক পরিসর, বিষয়, বিবরণ এবং ফুটারসহ একটি কমিট বার্তা গঠনে ধাপে ধাপে নির্দেশনা দেয়।

## বৈশিষ্ট্যসমূহ

- **মূল কার্যকারিতা**
  - কমিটের ধরন এবং পরিসরের ইন্টারেক্টিভ নির্বাচন
  - `.commitkit.toml`-এর মাধ্যমে কাস্টমাইজযোগ্য কনফিগারেশন
  - বিষয়ের সর্বোচ্চ দৈর্ঘ্য নিশ্চিতকরণ
  - বহু-লাইনের কমিট বিবরণের সমর্থন
  - ইনপুটের স্বয়ংক্রিয় যাচাইকরণ
  - কমিটে ঐচ্ছিক `Signed-off-by` লাইন যোগ করার সুবিধা
  - ক্রস-প্ল্যাটফর্ম সামঞ্জস্যতা

- **উন্নত বৈশিষ্ট্যসমূহ**
  - কমিট বার্তায় ইমোজি সমর্থন (✨, 🐛 ইত্যাদি)
  - প্লেসহোল্ডার প্রতিস্থাপনসহ কমিট টেমপ্লেট
  - স্বয়ংক্রিয় চেঞ্জলগ তৈরি ও হালনাগাদ
  - কমিট বার্তা যাচাই ও ফরম্যাটিংয়ের জন্য Git হুক
  - কমিট পরিসংখ্যান ও বিশ্লেষণ
  - টেমপ্লেট মানের জন্য ইন্টারেক্টিভ প্রম্পট

## পূর্বশর্তাবলি

- **Rust**: সংস্করণ 1.88.0 বা তার পরবর্তী (MSRV)
- **Cargo**: Rust ইনস্টলেশনের সাথে অন্তর্ভুক্ত

## ইনস্টলেশন

### সোর্স কোড থেকে

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## ব্যবহারবিধি

আপনার Git রিপোজিটরিতে প্রবেশ করে চালান:

```shell
commitkit
```

### কমান্ড-লাইন বিকল্পসমূহ

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           git commit এড়িয়ে গিয়ে বার্তাটি stdout-এ প্রিন্ট করুন
    -v, --verbose           বিস্তারিত আউটপুট দেখান
    -c, --config            কনফিগ ফাইলের পথ (ডিফল্ট: বর্তমান বা হোম ডিরেক্টরিতে .commitkit.toml)
        --emoji             কমিট বার্তায় ইমোজি চালু বা বন্ধ করুন
        --template          নির্দিষ্ট কমিট টেমপ্লেট ব্যবহার করুন
        --changelog         কমিট করার পর চেঞ্জলগ হালনাগাদ করুন
        --install-hooks     git হুক ইনস্টল করুন
        --stats             কমিট পরিসংখ্যান দেখান
        --days              পরিসংখ্যান বিশ্লেষণের জন্য দিনের সংখ্যা (ডিফল্ট: সব)
        --validate          একটি কমিট বার্তা ফাইল যাচাই করুন
        --prepare-msg       কমিট বার্তা প্রস্তুত করুন (git হুক দ্বারা ব্যবহৃত)
        --lang <LANG>       প্রদর্শন ভাষা (যেমন en, bn, hi, mr, ta)
    -h, --help              সহায়তা তথ্য প্রিন্ট করুন
    -V, --version           সংস্করণ তথ্য প্রিন্ট করুন
```

### সমর্থিত ভাষাসমূহ

CommitKit `LC_ALL`, `LC_MESSAGES` এবং `LANG` থেকে প্রদর্শন ভাষা শনাক্ত করে অথবা `--lang <LANG>` গ্রহণ করে। এটি ৪৫টি ভাষা সমর্থন করে, যার মধ্যে ডান-থেকে-বামে লেখা ৬টি ভাষা (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` এবং `ur`) অন্তর্ভুক্ত রয়েছে। সম্পূর্ণ তালিকার জন্য [ইংরেজি README](../../../README.md) দেখুন।

### ইন্টারেক্টিভ প্রক্রিয়া

এই টুলটি আপনাকে প্রচলিত নিয়ম অনুযায়ী কমিট বার্তা তৈরিতে নির্দেশনা দেবে:

1. কমিটের ধরন নির্বাচন করুন (`feat`, `fix`, `docs` ইত্যাদি)
2. পরিসর নির্বাচন করুন (ঐচ্ছিক)
3. কমিটের বিষয় লিখুন
4. কমিটের বিবরণ লিখুন (ঐচ্ছিক)
5. কমিটের ফুটার লিখুন (ঐচ্ছিক)

এই ধাপগুলো সম্পন্ন হলে CommitKit আপনার গঠিত বার্তা দিয়ে `git commit` চালাবে।

### উদাহরণ

```shell
# সাধারণ ব্যবহার
commitkit

# পরীক্ষামূলক চালনা - কমিট না করেই তৈরি হওয়া বার্তা দেখুন
commitkit --dry-run

# কাস্টম কনফিগ ফাইল ব্যবহার করুন
commitkit --config /path/to/custom-config.toml

# বিস্তারিত আউটপুট
commitkit --verbose

# কমিট বার্তায় ইমোজি ব্যবহার করুন
commitkit --emoji

# আপনার কমিটের জন্য টেমপ্লেট ব্যবহার করুন
commitkit --template feature

# স্বয়ংক্রিয়ভাবে CHANGELOG.md হালনাগাদ করুন
commitkit --changelog

# git হুক ইনস্টল করুন
commitkit --install-hooks

# গত ৩০ দিনের কমিট পরিসংখ্যান দেখুন
commitkit --stats --days 30

# একটি কমিট বার্তা যাচাই করুন
commitkit --validate .git/COMMIT_EDITMSG
```

## ডেভেলপমেন্ট

### বিল্ড

```bash
# ডিবাগ বিল্ড
cargo build

# রিলিজ বিল্ড
cargo build --release
```

### পরীক্ষা

```bash
# সব পরীক্ষা চালান
cargo test

# আউটপুটসহ পরীক্ষা চালান
cargo test -- --nocapture
```

### লিন্ট

```bash
# লিন্টিং সমস্যা পরীক্ষা করুন
cargo clippy --all-targets --all-features

# স্বয়ংক্রিয় সংশোধন প্রয়োগ করুন
cargo clippy --fix --allow-dirty
```

### ফরম্যাটিং

```bash
# ফরম্যাটিং পরীক্ষা করুন
cargo fmt -- --check

# ফরম্যাটিং প্রয়োগ করুন
cargo fmt
```

## কনফিগারেশন

CommitKit নিচের অবস্থানগুলোতে (ক্রমানুসারে) `.commitkit.toml` নামের কনফিগারেশন ফাইল অনুসন্ধান করে:

1. বর্তমান ডিরেক্টরি
2. ব্যবহারকারীর হোম ডিরেক্টরি

কোনো কনফিগারেশন ফাইল পাওয়া না গেলে, আপনার হোম ডিরেক্টরিতে একটি ডিফল্ট ফাইল তৈরি হবে।

উদাহরণ কনফিগারেশন:

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

## লাইসেন্স

এই প্রকল্পটি MIT লাইসেন্সের অধীনে প্রকাশিত — বিস্তারিত জানতে [LICENSE](../../../LICENSE) ফাইলটি দেখুন।

## অবদান

অবদান সর্বদা স্বাগত! নির্দ্বিধায় issue খুলুন অথবা pull request জমা দিন।

1. রিপোজিটরিটি ফর্ক করুন
2. আপনার ফিচার ব্রাঞ্চ তৈরি করুন (`git checkout -b feature/amazing-feature`)
3. প্রচলিত কমিট নিয়ম মেনে পরিবর্তনগুলো কমিট করুন
4. ব্রাঞ্চটি পুশ করুন (`git push origin feature/amazing-feature`)
5. একটি Pull Request খুলুন
