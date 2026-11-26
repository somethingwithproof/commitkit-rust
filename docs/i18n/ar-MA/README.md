[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit هي أداة سطر الأوامر كتعاون المطورين يصاوبو رسائل إيداع منظّمة ومتناسقة وفق معيار [Conventional Commits](https://www.conventionalcommits.org/). كتقدّم واجهة تفاعلية كترشد المستخدم خطوة بخطوة باش يركّب رسالة إيداع فيها النوع، النطاق الاختياري، العنوان، المحتوى، والتذييل.

## المميزات

- **الوظائف الأساسية**
  - اختيار تفاعلي لأنواع الإيداع والنطاقات
  - إعدادات قابلة للتخصيص عبر `.commitkit.toml`
  - احترام الحد الأقصى لطول العنوان
  - دعم محتوى الإيداع متعدد الأسطر
  - التحقق التلقائي من صحة المدخلات
  - إضافة سطر `Signed-off-by` اختياري للإيداعات
  - التوافق مع مختلف أنظمة التشغيل

- **مميزات متقدمة**
  - دعم الإيموجي فرسائل الإيداع (✨، 🐛، وغيرها)
  - قوالب إيداع مع تعويض المتغيرات
  - إنشاء وتحديث سجل التغييرات تلقائياً
  - خطافات Git للتحقق من رسائل الإيداع وتنسيقها
  - إحصائيات وتحليل الإيداعات
  - أسئلة تفاعلية لإدخال قيم القوالب

## المتطلبات القبلية

- **Rust**: النسخة 1.88.0 ولا أحدث (MSRV)
- **Cargo**: كيجي مدمج مع تثبيت Rust

## التثبيت

### من الشيفرة المصدرية

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## الاستعمال

دخل لمستودع Git ديالك وشغّل:

```shell
commitkit
```

### خيارات سطر الأوامر

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           تجاوز git commit وطبع الرسالة فـ stdout
    -v, --verbose           عرض المخرجات بالتفصيل
    -c, --config            مسار ملف الإعدادات (الافتراضي: .commitkit.toml فالمجلد الحالي ولا المجلد الرئيسي)
        --emoji             تفعيل ولا تعطيل الإيموجي فرسائل الإيداع
        --template          استعمال قالب إيداع محدد
        --changelog         تحديث سجل التغييرات من بعد الإيداع
        --install-hooks     تثبيت خطافات git
        --stats             عرض إحصائيات الإيداعات
        --days              عدد الأيام للتحليل فالإحصائيات (الافتراضي: الكل)
        --validate          التحقق من صحة ملف رسالة الإيداع
        --prepare-msg       تحضير رسالة الإيداع (كيستعملها خطاف git)
        --lang <LANG>       لغة العرض (مثلاً en, ar-MA, fr-MA, ar-EG, ja)
    -h, --help              عرض معلومات المساعدة
    -V, --version           عرض معلومات النسخة
```

### اللغات المدعومة

CommitKit كتكتشف لغة العرض أوتوماتيكياً من `LC_ALL` و`LC_MESSAGES` و`LANG`، ولا كتقبل الخيار `--lang <LANG>`. كتدعم 45 لغة، منها 6 لغات كتتكتب من اليمين لليسار (`ar`، `ar-EG`، `ar-MA`، `fa`، `he` و`ur`). شوف الجدول الكامل فـ [ملف README بالإنجليزية](../../../README.md).

### المسار التفاعلي

الأداة كترشدك فمراحل إنشاء رسالة إيداع تقليدية:

1. اختار نوع الإيداع (`feat`، `fix`، `docs`، إلخ)
2. اختار النطاق (اختياري)
3. دخل عنوان الإيداع
4. دخل محتوى الإيداع (اختياري)
5. دخل تذييل الإيداع (اختياري)

من بعد ما تسالي هاد الخطوات، CommitKit غادي تنفّذ `git commit` بالرسالة اللي صاوبتي.

### أمثلة

```shell
# الاستعمال الأساسي
commitkit

# وضع التجربة - عرض الرسالة الناتجة بلا ما يتدار إيداع
commitkit --dry-run

# استعمال ملف إعدادات مخصص
commitkit --config /path/to/custom-config.toml

# عرض المخرجات بالتفصيل
commitkit --verbose

# استعمال الإيموجي فرسائل الإيداع
commitkit --emoji

# استعمال قالب للإيداع
commitkit --template feature

# تحديث CHANGELOG.md تلقائياً
commitkit --changelog

# تثبيت خطافات git
commitkit --install-hooks

# عرض إحصائيات الإيداعات فآخر 30 يوم
commitkit --stats --days 30

# التحقق من صحة رسالة إيداع
commitkit --validate .git/COMMIT_EDITMSG
```

## التطوير

### البناء

```bash
# بناء للتصحيح
cargo build

# بناء للإصدار
cargo build --release
```

### الاختبارات

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل الاختبارات مع عرض المخرجات
cargo test -- --nocapture
```

### التدقيق

```bash
# التحقق من تحذيرات Clippy
cargo clippy --all-targets --all-features

# تطبيق الإصلاحات التلقائية
cargo clippy --fix --allow-dirty
```

### التنسيق

```bash
# التحقق من التنسيق
cargo fmt -- --check

# تطبيق التنسيق
cargo fmt
```

## الإعدادات

CommitKit كتقلب على ملف إعدادات سميتو `.commitkit.toml` فهاد المواقع (بالترتيب):

1. المجلد الحالي
2. المجلد الرئيسي ديال المستخدم

إلى ما لقاش ملف الإعدادات، غادي يتصاوب ملف افتراضي فالمجلد الرئيسي ديالك.

مثال على الإعدادات:

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

## الرخصة

هاد المشروع مرخّص برخصة MIT — راجع ملف [LICENSE](../../../LICENSE) للتفاصيل.

## المساهمة

المساهمات مرحب بيها! تقدر تفتح issue ولا تصيفط pull request.

1. دير Fork للمستودع
2. أنشئ فرع الميزة ديالك (`git checkout -b feature/amazing-feature`)
3. دير إيداع للتغييرات باستعمال الإيداعات التقليدية
4. دفع الفرع (`git push origin feature/amazing-feature`)
5. فتح Pull Request
