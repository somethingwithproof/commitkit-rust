[English](../../../README.md)

# CommitKit

[![CI](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/somethingwithproof/commitkit-rust/actions/workflows/ci.yml)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=somethingwithproof_commitkit-rust&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=somethingwithproof_commitkit-rust)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/commitkit-rust)](https://github.com/somethingwithproof/commitkit-rust/releases)
[![Rust minimum](https://img.shields.io/badge/Rust_minimum-1.88.0-blue)](../../../Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue)](../../../LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/commitkit-rust/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/commitkit-rust)

CommitKit เป็นเครื่องมือบรรทัดคำสั่ง (CLI) ที่ช่วยให้นักพัฒนาสร้างข้อความคอมมิตที่มีโครงสร้างชัดเจนและสอดคล้องกันตามมาตรฐาน [Conventional Commits](https://www.conventionalcommits.org/) โดยมีอินเทอร์เฟซแบบโต้ตอบคอยนำทางทีละขั้นตอนในการสร้างข้อความคอมมิตพร้อมประเภท ขอบเขต (ไม่บังคับ) หัวข้อ เนื้อหา และส่วนท้าย

## คุณสมบัติ

- **ความสามารถหลัก**
  - เลือกประเภทคอมมิตและขอบเขตแบบโต้ตอบ
  - ปรับแต่งการตั้งค่าผ่านไฟล์ `.commitkit.toml`
  - ควบคุมความยาวสูงสุดของหัวข้อคอมมิต
  - รองรับเนื้อหาคอมมิตแบบหลายบรรทัด
  - ตรวจสอบความถูกต้องของข้อมูลนำเข้าอัตโนมัติ
  - รองรับการลงชื่อกำกับคอมมิตแบบไม่บังคับ (`Signed-off-by`)
  - ใช้งานได้ข้ามแพลตฟอร์ม

- **คุณสมบัติเพิ่มเติม**
  - รองรับอีโมจิในข้อความคอมมิต (✨, 🐛 ฯลฯ)
  - เทมเพลตคอมมิตพร้อมการแทนที่ตัวแปร
  - สร้างและอัปเดตบันทึกการเปลี่ยนแปลงอัตโนมัติ
  - Git hooks สำหรับตรวจสอบและจัดรูปแบบข้อความคอมมิต
  - สถิติและการวิเคราะห์คอมมิต
  - คำถามแบบโต้ตอบสำหรับกรอกค่าในเทมเพลต

## ข้อกำหนดเบื้องต้น

- **Rust**: เวอร์ชัน 1.88.0 หรือสูงกว่า (MSRV)
- **Cargo**: มาพร้อมกับการติดตั้ง Rust

## การติดตั้ง

### จากซอร์สโค้ด

```shell
git clone https://github.com/somethingwithproof/commitkit-rust.git
cd commitkit-rust
cargo install --path .
```

## การใช้งาน

เข้าไปยังที่เก็บ Git ของคุณแล้วเรียกใช้คำสั่ง:

```shell
commitkit
```

### ตัวเลือกบรรทัดคำสั่ง

```text
USAGE:
    commitkit [OPTIONS]

OPTIONS:
    -d, --dry-run           ข้าม git commit แล้วพิมพ์ข้อความไปยัง stdout
    -v, --verbose           แสดงผลลัพธ์อย่างละเอียด
    -c, --config            เส้นทางไปยังไฟล์ตั้งค่า (ค่าเริ่มต้น: .commitkit.toml ในไดเรกทอรีปัจจุบันหรือโฮม)
        --emoji             เปิดหรือปิดการใช้อีโมจิในข้อความคอมมิต
        --template          ใช้เทมเพลตคอมมิตที่ระบุ
        --changelog         อัปเดตบันทึกการเปลี่ยนแปลงหลังคอมมิต
        --install-hooks     ติดตั้ง git hooks
        --stats             แสดงสถิติคอมมิต
        --days              จำนวนวันที่ต้องการวิเคราะห์สำหรับสถิติ (ค่าเริ่มต้น: ทั้งหมด)
        --validate          ตรวจสอบความถูกต้องของไฟล์ข้อความคอมมิต
        --prepare-msg       เตรียมข้อความคอมมิต (ใช้โดย git hook)
        --lang <LANG>       ภาษาที่แสดงผล (เช่น en, th, vi, ja, zh-CN)
    -h, --help              พิมพ์ข้อมูลวิธีใช้
    -V, --version           พิมพ์ข้อมูลเวอร์ชัน
```

### ภาษาที่รองรับ

CommitKit ตรวจจับภาษาที่แสดงผลอัตโนมัติจาก `LC_ALL`, `LC_MESSAGES` และ `LANG` หรือรับค่าจาก `--lang <LANG>` รองรับทั้งหมด 45 ภาษา รวมถึง 6 ภาษาที่เขียนจากขวาไปซ้าย (`ar`, `ar-EG`, `ar-MA`, `fa`, `he` และ `ur`) ดูตารางทั้งหมดได้ใน [README ภาษาอังกฤษ](../../../README.md)

### ขั้นตอนแบบโต้ตอบ

เครื่องมือจะนำทางคุณในการสร้างข้อความคอมมิตตามมาตรฐาน:

1. เลือกประเภทคอมมิต (`feat`, `fix`, `docs` ฯลฯ)
2. เลือกขอบเขต (ไม่บังคับ)
3. กรอกหัวข้อคอมมิต
4. กรอกเนื้อหาคอมมิต (ไม่บังคับ)
5. กรอกส่วนท้ายคอมมิต (ไม่บังคับ)

เมื่อดำเนินการครบทุกขั้นตอน CommitKit จะเรียกใช้ `git commit` ด้วยข้อความที่สร้างขึ้น

### ตัวอย่าง

```shell
# การใช้งานพื้นฐาน
commitkit

# โหมดทดลองรัน - แสดงข้อความคอมมิตที่สร้างขึ้นโดยไม่สร้างคอมมิตจริง
commitkit --dry-run

# ใช้ไฟล์ตั้งค่าที่กำหนดเอง
commitkit --config /path/to/custom-config.toml

# แสดงผลลัพธ์อย่างละเอียด
commitkit --verbose

# ใช้อีโมจิในข้อความคอมมิต
commitkit --emoji

# ใช้เทมเพลตสำหรับคอมมิตของคุณ
commitkit --template feature

# อัปเดต CHANGELOG.md อัตโนมัติ
commitkit --changelog

# ติดตั้ง git hooks
commitkit --install-hooks

# แสดงสถิติคอมมิตในช่วง 30 วันที่ผ่านมา
commitkit --stats --days 30

# ตรวจสอบข้อความคอมมิต
commitkit --validate .git/COMMIT_EDITMSG
```

## การพัฒนา

### การบิลด์

```bash
# ดีบักบิลด์
cargo build

# รีลีสบิลด์
cargo build --release
```

### การทดสอบ

```bash
# รันการทดสอบทั้งหมด
cargo test

# รันการทดสอบพร้อมแสดงผลลัพธ์
cargo test -- --nocapture
```

### การตรวจสอบโค้ด (Lint)

```bash
# ตรวจสอบคำเตือนจาก Clippy
cargo clippy --all-targets --all-features

# ใช้การแก้ไขอัตโนมัติ
cargo clippy --fix --allow-dirty
```

### การจัดรูปแบบโค้ด

```bash
# ตรวจสอบการจัดรูปแบบ
cargo fmt -- --check

# ใช้การจัดรูปแบบ
cargo fmt
```

## การตั้งค่า

CommitKit จะค้นหาไฟล์ตั้งค่าชื่อ `.commitkit.toml` ในตำแหน่งต่อไปนี้ตามลำดับ:

1. ไดเรกทอรีปัจจุบัน
2. ไดเรกทอรีโฮมของผู้ใช้

หากไม่พบไฟล์ตั้งค่า ระบบจะสร้างไฟล์ตั้งค่าเริ่มต้นไว้ในไดเรกทอรีโฮมของคุณโดยอัตโนมัติ

ตัวอย่างการตั้งค่า:

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

## สัญญาอนุญาต

โปรเจกต์นี้เผยแพร่ภายใต้สัญญาอนุญาต MIT — ดูรายละเอียดเพิ่มเติมได้ที่ไฟล์ [LICENSE](../../../LICENSE)

## การมีส่วนร่วม

ยินดีต้อนรับทุกการมีส่วนร่วม! สามารถเปิด issue หรือส่ง pull request ได้เสมอ

1. Fork ที่เก็บนี้
2. สร้างแบรนช์ฟีเจอร์ของคุณ (`git checkout -b feature/amazing-feature`)
3. คอมมิตการเปลี่ยนแปลงตามมาตรฐาน conventional commits
4. Push แบรนช์ขึ้นไป (`git push origin feature/amazing-feature`)
5. เปิด Pull Request
