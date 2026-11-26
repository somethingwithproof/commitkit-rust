[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit הוא כלי שורת פקודה המסייע למפתחים ליצור הודעות קומיט מובנות ועקביות בהתאם למפרט [Conventional Commits](https://www.conventionalcommits.org/). הכלי מספק ממשק אינטראקטיבי המדריך את המשתמש שלב אחר שלב בבניית הודעת קומיט הכוללת סוג, תחום אופציונלי, נושא, גוף וכותרת תחתונה.

## תכונות

- **פונקציונליות ליבה**
  - בחירה אינטראקטיבית של סוגי קומיט ותחומים
  - תצורה הניתנת להתאמה אישית באמצעות `.commitkit.toml`
  - אכיפת אורך מרבי של שורת הנושא
  - תמיכה בגוף קומיט רב-שורתי
  - אימות אוטומטי של הקלט
  - אפשרות לחתימת קומיטים (`Signed-off-by`)
  - תאימות חוצת פלטפורמות

- **תכונות מתקדמות**
  - תמיכה באימוג׳ים בהודעות קומיט (✨, 🐛 ועוד)
  - תבניות קומיט עם החלפת מצייני מקום
  - יצירה ועדכון אוטומטיים של יומן שינויים
  - Git hooks לאימות ועיצוב הודעות קומיט
  - סטטיסטיקות וניתוח קומיטים
  - הנחיות אינטראקטיביות להזנת ערכי תבניות

## דרישות קדם

- **Rust**: גרסה 1.88.0 ומעלה (MSRV)
- **Cargo**: כלול בהתקנת Rust

## התקנה

### מקוד המקור

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## שימוש

נווטו אל מאגר ה-Git שלכם והריצו:

```shell
commitkit
```

### אפשרויות שורת הפקודה

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           דלג על git commit והדפס את ההודעה אל stdout
    -v, --verbose           הצג פלט מפורט
    -c, --config            נתיב לקובץ התצורה (ברירת מחדל: .commitkit.toml בתיקייה הנוכחית או בתיקיית הבית)
        --emoji             הפעל או כבה אימוג׳ים בהודעות קומיט
        --template          השתמש בתבנית קומיט מסוימת
        --changelog         עדכן את יומן השינויים לאחר הקומיט
        --install-hooks     התקן git hooks
        --stats             הצג סטטיסטיקות קומיטים
        --days              מספר הימים לניתוח הסטטיסטיקות (ברירת מחדל: הכל)
        --validate          אמת קובץ הודעת קומיט
        --prepare-msg       הכן הודעת קומיט (בשימוש של git hook)
        --lang <LANG>       שפת תצוגה (למשל en, he, ar, fa, ja)
    -h, --help              הדפס מידע עזרה
    -V, --version           הדפס מידע גרסה
```

### שפות נתמכות

CommitKit מזהה את שפת התצוגה ממשתני הסביבה `LC_ALL`, `LC_MESSAGES` ו-`LANG`, או מקבל `--lang <LANG>`. הכלי תומך ב-45 שפות, כולל 6 שפות מימין לשמאל (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` ו-`ur`). לטבלה המלאה ראו את [קובץ ה-README באנגלית](../../../README.md).

### תהליך אינטראקטיבי

הכלי ידריך אתכם ביצירת הודעת קומיט מוסכמת:

1. בחירת סוג קומיט (`feat`, `fix`, `docs` ועוד)
2. בחירת תחום (אופציונלי)
3. הזנת נושא הקומיט
4. הזנת גוף הקומיט (אופציונלי)
5. הזנת כותרת תחתונה לקומיט (אופציונלי)

לאחר השלמת השלבים הללו, CommitKit יריץ `git commit` עם ההודעה שנוצרה.

### דוגמאות

```shell
# שימוש בסיסי
commitkit

# הרצה יבשה - הצגת הודעת הקומיט מבלי לבצע קומיט
commitkit --dry-run

# שימוש בקובץ תצורה מותאם אישית
commitkit --config /path/to/custom-config.toml

# פלט מפורט
commitkit --verbose

# שימוש באימוג׳ים בהודעות קומיט
commitkit --emoji

# שימוש בתבנית עבור הקומיט
commitkit --template feature

# עדכון אוטומטי של CHANGELOG.md
commitkit --changelog

# התקנת git hooks
commitkit --install-hooks

# הצגת סטטיסטיקות קומיטים ל-30 הימים האחרונים
commitkit --stats --days 30

# אימות הודעת קומיט
commitkit --validate .git/COMMIT_EDITMSG
```

## פיתוח

### בנייה

```bash
# בניית ניפוי שגיאות (Debug)
cargo build

# בניית הפצה (Release)
cargo build --release
```

### בדיקות

```bash
# הרצת כל הבדיקות
cargo test

# הרצת בדיקות עם הצגת פלט
cargo test -- --nocapture
```

### בדיקת קוד (Lint)

```bash
# בדיקת אזהרות Clippy
cargo clippy --all-targets --all-features

# החלת תיקונים אוטומטיים
cargo clippy --fix --allow-dirty
```

### עיצוב קוד

```bash
# בדיקת עיצוב
cargo fmt -- --check

# החלת עיצוב
cargo fmt
```

## תצורה

CommitKit מחפש קובץ תצורה בשם `.commitkit.toml` במיקומים הבאים (לפי הסדר):

1. התיקייה הנוכחית
2. תיקיית הבית של המשתמש

אם לא נמצא קובץ תצורה, ייווצר קובץ ברירת מחדל בתיקיית הבית שלכם.

דוגמת תצורה:

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

## רישיון

פרויקט זה מופץ תחת רישיון MIT — לפרטים ראו את הקובץ [LICENSE](../../../LICENSE).

## תרומה לפרויקט

תרומות מתקבלות בברכה! אתם מוזמנים לפתוח issues או להגיש pull requests.

1. בצעו Fork למאגר
2. צרו ענף תכונה חדש (`git checkout -b feature/amazing-feature`)
3. שמרו את השינויים בקומיטים לפי Conventional Commits
4. דחפו את הענף (`git push origin feature/amazing-feature`)
5. פתחו Pull Request
