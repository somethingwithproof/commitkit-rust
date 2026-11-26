[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit یک ابزار خط فرمان است که به توسعه‌دهندگان کمک می‌کند پیام‌های کامیت ساختاریافته و یکنواخت را مطابق با استاندارد [Conventional Commits](https://www.conventionalcommits.org/) ایجاد کنند. این ابزار یک رابط تعاملی ارائه می‌دهد که گام‌به‌گام شما را در ساخت پیام کامیت شامل نوع، حوزه اختیاری، عنوان، متن و پاورقی راهنمایی می‌کند.

## ویژگی‌ها

- **قابلیت‌های اصلی**
  - انتخاب تعاملی نوع کامیت و حوزه‌ها
  - پیکربندی قابل سفارشی‌سازی از طریق `.commitkit.toml`
  - اعمال محدودیت حداکثر طول عنوان
  - پشتیبانی از متن کامیت چندخطی
  - اعتبارسنجی خودکار ورودی‌ها
  - امکان امضای اختیاری کامیت‌ها (`Signed-off-by`)
  - سازگاری با سیستم‌عامل‌های مختلف

- **قابلیت‌های پیشرفته**
  - پشتیبانی از ایموجی در پیام‌های کامیت (✨، 🐛 و غیره)
  - الگوهای کامیت با جایگزینی متغیرها
  - تولید و به‌روزرسانی خودکار فهرست تغییرات
  - قلاب‌های Git برای اعتبارسنجی و قالب‌بندی پیام‌های کامیت
  - آمار و تحلیل کامیت‌ها
  - پرسش‌های تعاملی برای مقادیر الگوها

## پیش‌نیازها

- **Rust**: نسخه 1.88.0 یا بالاتر (MSRV)
- **Cargo**: همراه با نصب Rust ارائه می‌شود

## نصب

### از کد منبع

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## نحوه استفاده

به مخزن Git خود بروید و دستور زیر را اجرا کنید:

```shell
commitkit
```

### گزینه‌های خط فرمان

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           رد کردن git commit و چاپ پیام در stdout
    -v, --verbose           نمایش خروجی با جزئیات کامل
    -c, --config            مسیر فایل پیکربندی (پیش‌فرض: .commitkit.toml در پوشه جاری یا پوشه خانگی)
        --emoji             فعال یا غیرفعال کردن ایموجی در پیام‌های کامیت
        --template          استفاده از یک الگوی کامیت مشخص
        --changelog         به‌روزرسانی فهرست تغییرات پس از کامیت
        --install-hooks     نصب قلاب‌های git
        --stats             نمایش آمار کامیت‌ها
        --days              تعداد روزها برای تحلیل آمار (پیش‌فرض: همه)
        --validate          اعتبارسنجی فایل پیام کامیت
        --prepare-msg       آماده‌سازی پیام کامیت (مورد استفاده قلاب git)
        --lang <LANG>       زبان نمایش (مثلاً en, fa, ar, ur, ja)
    -h, --help              نمایش اطلاعات راهنما
    -V, --version           نمایش اطلاعات نسخه
```

### زبان‌های پشتیبانی‌شده

CommitKit زبان نمایش را از متغیرهای محیطی `LC_ALL`، `LC_MESSAGES` و `LANG` تشخیص می‌دهد یا گزینه `--lang <LANG>` را می‌پذیرد. این ابزار از ۴۵ زبان، از جمله ۶ زبان راست‌به‌چپ (`ar`، `ar-EG`، `ar-MA`، `fa`، `he` و `ur`) پشتیبانی می‌کند. جدول کامل را در [فایل README انگلیسی](../../../README.md) مشاهده کنید.

### فرایند تعاملی

این ابزار شما را در ایجاد یک پیام کامیت استاندارد راهنمایی می‌کند:

1. نوع کامیت را انتخاب کنید (`feat`، `fix`، `docs` و غیره)
2. حوزه را انتخاب کنید (اختیاری)
3. عنوان کامیت را وارد کنید
4. متن کامیت را وارد کنید (اختیاری)
5. پاورقی کامیت را وارد کنید (اختیاری)

پس از تکمیل این مراحل، CommitKit دستور `git commit` را با پیام ساخته‌شده اجرا می‌کند.

### مثال‌ها

```shell
# استفاده پایه
commitkit

# حالت آزمایشی - نمایش پیام کامیت بدون ثبت واقعی کامیت
commitkit --dry-run

# استفاده از فایل پیکربندی سفارشی
commitkit --config /path/to/custom-config.toml

# خروجی با جزئیات
commitkit --verbose

# استفاده از ایموجی در پیام‌های کامیت
commitkit --emoji

# استفاده از الگو برای کامیت
commitkit --template feature

# به‌روزرسانی خودکار CHANGELOG.md
commitkit --changelog

# نصب قلاب‌های git
commitkit --install-hooks

# نمایش آمار کامیت‌ها در ۳۰ روز گذشته
commitkit --stats --days 30

# اعتبارسنجی یک پیام کامیت
commitkit --validate .git/COMMIT_EDITMSG
```

## توسعه

### ساخت

```bash
# ساخت اشکال‌زدایی (Debug)
cargo build

# ساخت انتشار (Release)
cargo build --release
```

### آزمون

```bash
# اجرای تمام آزمون‌ها
cargo test

# اجرای آزمون‌ها همراه با نمایش خروجی
cargo test -- --nocapture
```

### بررسی کد (Lint)

```bash
# بررسی هشدارهای Clippy
cargo clippy --all-targets --all-features

# اعمال اصلاحات خودکار
cargo clippy --fix --allow-dirty
```

### قالب‌بندی کد

```bash
# بررسی قالب‌بندی
cargo fmt -- --check

# اعمال قالب‌بندی
cargo fmt
```

## پیکربندی

CommitKit به دنبال فایل پیکربندی با نام `.commitkit.toml` در مسیرهای زیر (به ترتیب) می‌گردد:

1. پوشه جاری
2. پوشه خانگی کاربر

اگر هیچ فایل پیکربندی یافت نشود، یک فایل پیش‌فرض در پوشه خانگی شما ایجاد خواهد شد.

نمونه پیکربندی:

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

## مجوز

این پروژه تحت مجوز MIT منتشر شده است — برای جزئیات بیشتر فایل [LICENSE](../../../LICENSE) را ببینید.

## مشارکت

از مشارکت شما استقبال می‌کنیم! می‌توانید issue ثبت کنید یا pull request ارسال نمایید.

1. مخزن را Fork کنید
2. شاخه ویژگی خود را بسازید (`git checkout -b feature/amazing-feature`)
3. تغییرات خود را با قالب کامیت‌های استاندارد ثبت کنید
4. شاخه را Push کنید (`git push origin feature/amazing-feature`)
5. یک Pull Request باز کنید
