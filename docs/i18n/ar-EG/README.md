[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit هي أداة سطر أوامر بتساعد المطورين يكتبوا رسائل إيداع منظمة ومتناسقة حسب معيار [Conventional Commits](https://www.conventionalcommits.org/). بتقدّم واجهة تفاعلية بتمشي معاك خطوة بخطوة عشان تكوّن رسالة إيداع فيها النوع والنطاق الاختياري والعنوان والتفاصيل والتذييل.

## المميزات

- **الوظائف الأساسية**
  - اختيار تفاعلي لأنواع الإيداع والنطاقات
  - إعدادات مرنة قابلة للتخصيص من خلال `.commitkit.toml`
  - الالتزام بالحد الأقصى لطول العنوان
  - دعم تفاصيل الإيداع متعددة السطور
  - التحقق التلقائي من صحة المدخلات
  - إضافة سطر `Signed-off-by` اختياري للإيداعات
  - متوافقة مع كل أنظمة التشغيل

- **مميزات إضافية**
  - دعم الإيموجي في رسائل الإيداع (✨، 🐛، وغيرها)
  - قوالب إيداع مع استبدال المتغيرات
  - إنشاء وتحديث سجل التغييرات تلقائياً
  - خطافات Git للتحقق من رسائل الإيداع وتنسيقها
  - إحصائيات وتحليل الإيداعات
  - أسئلة تفاعلية لإدخال قيم القوالب

## المتطلبات

- **Rust**: إصدار 1.88.0 أو أحدث (MSRV)
- **Cargo**: بيجي مع تثبيت Rust

## التثبيت

### من الكود المصدري

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## الاستخدام

ادخل على مستودع Git بتاعك وشغّل:

```shell
commitkit
```

### خيارات سطر الأوامر

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           تخطي git commit واطبع الرسالة على stdout
    -v, --verbose           اعرض تفاصيل أكتر في المخرجات
    -c, --config            مسار ملف الإعدادات (الافتراضي: .commitkit.toml في المجلد الحالي أو المجلد الرئيسي)
        --emoji             شغّل أو وقّف الإيموجي في رسائل الإيداع
        --template          استخدم قالب إيداع محدد
        --changelog         حدّث سجل التغييرات بعد الإيداع
        --install-hooks     ثبّت خطافات git
        --stats             اعرض إحصائيات الإيداعات
        --days              عدد الأيام لتحليل الإحصائيات (الافتراضي: الكل)
        --validate          اتأكد من صحة ملف رسالة الإيداع
        --prepare-msg       جهّز رسالة الإيداع (بيستخدمها خطاف git)
        --lang <LANG>       لغة العرض (مثلاً en, ar-EG, ar-MA, fr, ja)
    -h, --help              اعرض معلومات المساعدة
    -V, --version           اعرض رقم الإصدار
```

### اللغات المدعومة

CommitKit بتتعرف تلقائياً على لغة العرض من `LC_ALL` و`LC_MESSAGES` و`LANG`، أو بتقبل الخيار `--lang <LANG>`. الأداة بتدعم 45 لغة، منهم 6 لغات بتتكتب من اليمين للشمال (`ar` و`ar-EG` و`ar-MA` و`fa` و`he` و`ur`). شوف الجدول الكامل في [ملف README بالإنجليزي](../../../README.md).

### الخطوات التفاعلية

الأداة هتمشي معاك في خطوات عمل رسالة إيداع تقليدية:

1. اختار نوع الإيداع (`feat`، `fix`، `docs`، إلخ)
2. اختار النطاق (اختياري)
3. اكتب عنوان الإيداع
4. اكتب تفاصيل الإيداع (اختياري)
5. اكتب تذييل الإيداع (اختياري)

بعد ما تخلّص الخطوات دي، CommitKit هتنفّذ `git commit` بالرسالة اللي جهّزتها.

### أمثلة

```shell
# الاستخدام الأساسي
commitkit

# وضع التجربة - اعرض الرسالة الناتجة من غير ما تعمل إيداع فعلي
commitkit --dry-run

# استخدم ملف إعدادات مخصص
commitkit --config /path/to/custom-config.toml

# مخرجات تفصيلية
commitkit --verbose

# استخدم الإيموجي في رسائل الإيداع
commitkit --emoji

# استخدم قالب للإيداع
commitkit --template feature

# حدّث CHANGELOG.md تلقائياً
commitkit --changelog

# ثبّت خطافات git
commitkit --install-hooks

# اعرض إحصائيات الإيداعات لآخر 30 يوم
commitkit --stats --days 30

# اتأكد من صحة رسالة إيداع
commitkit --validate .git/COMMIT_EDITMSG
```

## التطوير

### البناء

```bash
# بناء للتجربة والتصحيح
cargo build

# بناء للإصدار النهائي
cargo build --release
```

### الاختبارات

```bash
# شغّل كل الاختبارات
cargo test

# شغّل الاختبارات مع عرض المخرجات
cargo test -- --nocapture
```

### فحص الكود

```bash
# افحص تحذيرات Clippy
cargo clippy --all-targets --all-features

# طبّق الإصلاحات التلقائية
cargo clippy --fix --allow-dirty
```

### التنسيق

```bash
# افحص التنسيق
cargo fmt -- --check

# طبّق التنسيق
cargo fmt
```

## الإعدادات

CommitKit بتدور على ملف إعدادات اسمه `.commitkit.toml` في الأماكن دي (بالترتيب):

1. المجلد الحالي
2. المجلد الرئيسي للمستخدم

لو مفيش ملف إعدادات، الأداة هتعمل ملف افتراضي في مجلدك الرئيسي.

مثال على ملف الإعدادات:

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

المشروع ده مرخّص برخصة MIT — راجع ملف [LICENSE](../../../LICENSE) للتفاصيل.

## المساهمة

المساهمات مرحب بيها دايماً! تقدر تفتح issue أو تبعت pull request في أي وقت.

1. اعمل Fork للمستودع
2. اعمل فرع للميزة الجديدة (`git checkout -b feature/amazing-feature`)
3. اعمل إيداع لتغييراتك باستخدام معيار Conventional Commits
4. ارفع الفرع (`git push origin feature/amazing-feature`)
5. افتح Pull Request
