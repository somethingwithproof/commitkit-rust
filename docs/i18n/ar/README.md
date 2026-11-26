[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit هي أداة سطر أوامر تساعد المطورين على إنشاء رسائل إيداع منظمة ومتسقة وفقاً لمواصفات [Conventional Commits](https://www.conventionalcommits.org/). توفر الأداة واجهة تفاعلية ترشد المستخدمين خطوة بخطوة لصياغة رسالة إيداع تتضمن النوع والنطاق الاختياري والعنوان والمتن والتذييل.

## الميزات

- **الوظائف الأساسية**
  - اختيار تفاعلي لأنواع الإيداع والنطاقات
  - إعدادات قابلة للتخصيص عبر ملف `.commitkit.toml`
  - فرض الحد الأقصى لطول العنوان
  - دعم متون الإيداع متعددة الأسطر
  - التحقق التلقائي من صحة المدخلات
  - إضافة سطر `Signed-off-by` اختيارياً إلى الإيداعات
  - التوافق عبر مختلف أنظمة التشغيل

- **الميزات المتقدمة**
  - دعم الرموز التعبيرية في رسائل الإيداع (✨، 🐛، وغيرها)
  - قوالب إيداع مع استبدال المتغيرات
  - إنشاء سجل التغييرات وتحديثه تلقائياً
  - خطافات Git للتحقق من صحة رسائل الإيداع وتنسيقها
  - إحصائيات الإيداعات وتحليلها
  - مطالبات تفاعلية لإدخال قيم القوالب

## المتطلبات الأساسية

- **Rust**: الإصدار 1.88.0 أو أحدث (MSRV)
- **Cargo**: مضمّن مع تثبيت Rust

## التثبيت

### من الشيفرة المصدرية

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## الاستخدام

انتقل إلى مستودع Git الخاص بك ونفّذ الأمر التالي:

```shell
commitkit
```

### خيارات سطر الأوامر

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           تخطي git commit وطباعة الرسالة إلى المخرج القياسي
    -v, --verbose           عرض مخرجات مفصّلة
    -c, --config            مسار ملف الإعدادات (الافتراضي: .commitkit.toml في المجلد الحالي أو الرئيسي)
        --emoji             تبديل الرموز التعبيرية في رسائل الإيداع
        --template          استخدام قالب إيداع محدد
        --changelog         تحديث سجل التغييرات بعد الإيداع
        --install-hooks     تثبيت خطافات git
        --stats             عرض إحصائيات الإيداعات
        --days              عدد الأيام لتحليلها في الإحصائيات (الافتراضي: الكل)
        --validate          التحقق من صحة ملف رسالة الإيداع
        --prepare-msg       تحضير رسالة الإيداع (يستخدمه خطاف git)
        --lang <LANG>       لغة العرض (مثل en, de, ja, zh-CN, pt-BR)
    -h, --help              طباعة معلومات المساعدة
    -V, --version           طباعة معلومات الإصدار
```

### اللغات المدعومة

تكتشف CommitKit لغة العرض تلقائياً من متغيرات البيئة `LC_ALL` و`LC_MESSAGES` و`LANG`، كما تقبل الخيار `--lang <LANG>`. تدعم الأداة 45 لغة، منها 6 لغات تُكتب من اليمين إلى اليسار (`ar` و`ar-EG` و`ar-MA` و`fa` و`he` و`ur`). راجع الجدول الكامل في [ملف README بالإنجليزية](../../../README.md).

### العملية التفاعلية

ترشدك الأداة خلال خطوات إنشاء رسالة إيداع تقليدية:

1. اختيار نوع الإيداع (`feat`، `fix`، `docs`، إلخ)
2. اختيار النطاق (اختياري)
3. إدخال عنوان الإيداع
4. إدخال متن الإيداع (اختياري)
5. إدخال تذييل الإيداع (اختياري)

بعد إكمال هذه الخطوات، ستنفذ CommitKit الأمر `git commit` بالرسالة التي أنشأتها.

### أمثلة

```shell
# الاستخدام الأساسي
commitkit

# تشغيل تجريبي - عرض رسالة الإيداع الناتجة دون تنفيذ الإيداع
commitkit --dry-run

# استخدام ملف إعدادات مخصص
commitkit --config /path/to/custom-config.toml

# مخرجات مفصّلة
commitkit --verbose

# استخدام الرموز التعبيرية في رسائل الإيداع
commitkit --emoji

# استخدام قالب للإيداع
commitkit --template feature

# تحديث CHANGELOG.md تلقائياً
commitkit --changelog

# تثبيت خطافات git
commitkit --install-hooks

# عرض إحصائيات الإيداعات لآخر 30 يوماً
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

### الاختبار

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل الاختبارات مع عرض المخرجات
cargo test -- --nocapture
```

### فحص جودة الشيفرة

```bash
# التحقق من مشكلات التدقيق
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

تبحث CommitKit عن ملف إعدادات باسم `.commitkit.toml` في المواقع التالية (بالترتيب):

1. المجلد الحالي
2. المجلد الرئيسي للمستخدم

في حال عدم العثور على ملف إعدادات، سيتم إنشاء ملف افتراضي في مجلدك الرئيسي.

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

هذا المشروع مرخّص بموجب رخصة MIT — راجع ملف [LICENSE](../../../LICENSE) للتفاصيل.

## المساهمة

المساهمات مرحّب بها دائماً! لا تتردد في فتح المشكلات أو إرسال طلبات السحب.

1. انسخ المستودع (Fork)
2. أنشئ فرع الميزة الخاص بك (`git checkout -b feature/amazing-feature`)
3. أودع تغييراتك وفق معيار الإيداعات التقليدية
4. ادفع الفرع (`git push origin feature/amazing-feature`)
5. افتح طلب سحب (Pull Request)
