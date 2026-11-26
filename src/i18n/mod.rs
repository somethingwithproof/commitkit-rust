// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

//! Internationalization (i18n) support for CommitKit.
//!
//! Provides compile-time checked translations for 45 locales (including 6 RTL
//! locales) with POSIX environment detection (`LC_ALL` -> `LC_MESSAGES` -> `LANG`)
//! and optional explicit override via `--lang`.

pub mod catalog;

pub use catalog::Catalog;

/// Supported display locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    #[default]
    En,
    De,
    Es,
    Fr,
    It,
    PtBr,
    Nl,
    Ru,
    Uk,
    Ja,
    Ko,
    ZhCn,
    ZhTw,
    Ar,
    Hi,
    Pl,
    Tr,
    Sv,
    Vi,
    Id,
    AfZa,
    AmEt,
    ArEg,
    ArMa,
    FrMa,
    FrSn,
    HaNg,
    IgNg,
    MgMg,
    RwRw,
    SoSo,
    SwKe,
    XhZa,
    YoNg,
    ZuZa,
    Bn,
    Fa,
    He,
    Mr,
    Ta,
    Te,
    Th,
    Ur,
    Ms,
    Fil,
}

impl Locale {
    /// All 45 supported locales.
    pub const ALL: [Locale; 45] = [
        Locale::En,
        Locale::De,
        Locale::Es,
        Locale::Fr,
        Locale::It,
        Locale::PtBr,
        Locale::Nl,
        Locale::Ru,
        Locale::Uk,
        Locale::Ja,
        Locale::Ko,
        Locale::ZhCn,
        Locale::ZhTw,
        Locale::Ar,
        Locale::Hi,
        Locale::Pl,
        Locale::Tr,
        Locale::Sv,
        Locale::Vi,
        Locale::Id,
        Locale::AfZa,
        Locale::AmEt,
        Locale::ArEg,
        Locale::ArMa,
        Locale::FrMa,
        Locale::FrSn,
        Locale::HaNg,
        Locale::IgNg,
        Locale::MgMg,
        Locale::RwRw,
        Locale::SoSo,
        Locale::SwKe,
        Locale::XhZa,
        Locale::YoNg,
        Locale::ZuZa,
        Locale::Bn,
        Locale::Fa,
        Locale::He,
        Locale::Mr,
        Locale::Ta,
        Locale::Te,
        Locale::Th,
        Locale::Ur,
        Locale::Ms,
        Locale::Fil,
    ];

    /// Canonical BCP 47 language tag for this locale.
    pub const fn code(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::It => "it",
            Locale::PtBr => "pt-BR",
            Locale::Nl => "nl",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::Ja => "ja",
            Locale::Ko => "ko",
            Locale::ZhCn => "zh-CN",
            Locale::ZhTw => "zh-TW",
            Locale::Ar => "ar",
            Locale::Hi => "hi",
            Locale::Pl => "pl",
            Locale::Tr => "tr",
            Locale::Sv => "sv",
            Locale::Vi => "vi",
            Locale::Id => "id",
            Locale::AfZa => "af-ZA",
            Locale::AmEt => "am-ET",
            Locale::ArEg => "ar-EG",
            Locale::ArMa => "ar-MA",
            Locale::FrMa => "fr-MA",
            Locale::FrSn => "fr-SN",
            Locale::HaNg => "ha-NG",
            Locale::IgNg => "ig-NG",
            Locale::MgMg => "mg-MG",
            Locale::RwRw => "rw-RW",
            Locale::SoSo => "so-SO",
            Locale::SwKe => "sw-KE",
            Locale::XhZa => "xh-ZA",
            Locale::YoNg => "yo-NG",
            Locale::ZuZa => "zu-ZA",
            Locale::Bn => "bn",
            Locale::Fa => "fa",
            Locale::He => "he",
            Locale::Mr => "mr",
            Locale::Ta => "ta",
            Locale::Te => "te",
            Locale::Th => "th",
            Locale::Ur => "ur",
            Locale::Ms => "ms",
            Locale::Fil => "fil",
        }
    }

    /// Returns `true` if this locale uses a right-to-left (RTL) script.
    pub const fn is_rtl(self) -> bool {
        matches!(
            self,
            Locale::Ar | Locale::ArEg | Locale::ArMa | Locale::Fa | Locale::He | Locale::Ur
        )
    }

    /// Parse a BCP 47 or POSIX locale tag (e.g. `de_DE.UTF-8`, `ar_EG.UTF-8`,
    /// `fr-Latn-SN`, `zh_Hans_CN`, `iw_IL`, `tl_PH`, `C`).
    pub fn from_tag(tag: &str) -> Option<Locale> {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return None;
        }

        // Strip POSIX encoding (`.UTF-8`) and modifier (`@euro`).
        let base = trimmed
            .split('@')
            .next()
            .unwrap_or(trimmed)
            .split('.')
            .next()
            .unwrap_or(trimmed);

        let normalized = base.replace('_', "-").to_ascii_lowercase();

        if normalized == "c" || normalized == "posix" {
            return Some(Locale::En);
        }

        let mut parts = normalized.split('-');
        let primary = parts.next().unwrap_or("");

        // Parse core script and region subtags only, stopping before any BCP 47
        // singleton-led extension (`u`, `t`, ...) or private-use (`x`) section.
        let mut script: Option<&str> = None;
        let mut region: Option<&str> = None;
        for subtag in parts {
            if subtag.len() == 1 {
                break;
            }
            if script.is_none()
                && region.is_none()
                && subtag.len() == 4
                && subtag.bytes().all(|b| b.is_ascii_alphabetic())
            {
                script = Some(subtag);
            } else if region.is_none()
                && ((subtag.len() == 2 && subtag.bytes().all(|b| b.is_ascii_alphabetic()))
                    || (subtag.len() == 3 && subtag.bytes().all(|b| b.is_ascii_digit())))
            {
                region = Some(subtag);
            }
        }

        match primary {
            "en" => Some(Locale::En),
            "de" => Some(Locale::De),
            "es" => Some(Locale::Es),
            "fr" => match region {
                Some("ma") => Some(Locale::FrMa),
                Some("sn") => Some(Locale::FrSn),
                _ => Some(Locale::Fr),
            },
            "it" => Some(Locale::It),
            "pt" => Some(Locale::PtBr),
            "nl" => Some(Locale::Nl),
            "ru" => Some(Locale::Ru),
            "uk" => Some(Locale::Uk),
            "ja" => Some(Locale::Ja),
            "ko" => Some(Locale::Ko),
            "zh" => match script {
                Some("hans") => Some(Locale::ZhCn),
                Some("hant") => Some(Locale::ZhTw),
                _ => match region {
                    Some("tw" | "hk" | "mo") => Some(Locale::ZhTw),
                    _ => Some(Locale::ZhCn),
                },
            },
            "ar" => match region {
                Some("eg") => Some(Locale::ArEg),
                Some("ma") => Some(Locale::ArMa),
                _ => Some(Locale::Ar),
            },
            "hi" => Some(Locale::Hi),
            "pl" => Some(Locale::Pl),
            "tr" => Some(Locale::Tr),
            "sv" => Some(Locale::Sv),
            "vi" => Some(Locale::Vi),
            "id" | "in" => Some(Locale::Id),
            "af" => Some(Locale::AfZa),
            "am" => Some(Locale::AmEt),
            "ha" => Some(Locale::HaNg),
            "ig" => Some(Locale::IgNg),
            "mg" => Some(Locale::MgMg),
            "rw" => Some(Locale::RwRw),
            "so" => Some(Locale::SoSo),
            "sw" => Some(Locale::SwKe),
            "xh" => Some(Locale::XhZa),
            "yo" => Some(Locale::YoNg),
            "zu" => Some(Locale::ZuZa),
            "bn" => Some(Locale::Bn),
            "fa" => Some(Locale::Fa),
            "he" | "iw" => Some(Locale::He),
            "mr" => Some(Locale::Mr),
            "ta" => Some(Locale::Ta),
            "te" => Some(Locale::Te),
            "th" => Some(Locale::Th),
            "ur" => Some(Locale::Ur),
            "ms" => Some(Locale::Ms),
            "fil" | "tl" => Some(Locale::Fil),
            _ => None,
        }
    }

    /// Detect locale from POSIX environment variables (`LC_ALL` -> `LC_MESSAGES` -> `LANG`).
    pub fn detect() -> Locale {
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Ok(value) = std::env::var(var) {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(locale) = Self::from_tag(trimmed) {
                    return locale;
                }
                return Locale::En;
            }
        }
        Locale::En
    }

    /// Resolve locale from an optional explicit tag, falling back to environment detection.
    pub fn resolve(explicit: Option<&str>) -> Locale {
        if let Some(tag) = explicit {
            if let Some(locale) = Self::from_tag(tag) {
                return locale;
            }
            return Locale::En;
        }
        Self::detect()
    }

    /// Return the static message catalog for this locale.
    pub const fn catalog(self) -> &'static Catalog {
        catalog::for_locale(self)
    }
}

/// Extract `--lang <value>` or `--lang=<value>` from raw CLI arguments so `--help`
/// output can be localized before full argument parsing runs.
pub fn extract_cli_lang(args: &[String]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        if arg == "--lang" {
            return iter.next().cloned();
        }
        if let Some(val) = arg.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Extract `--lang <value>` or `--lang=<value>` from OS-native CLI arguments so
/// `--help` output can be localized without panicking on non-UTF-8 arguments.
pub fn extract_cli_lang_os(args: &[std::ffi::OsString]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        let Some(arg_str) = arg.to_str() else {
            continue;
        };
        if arg_str == "--" {
            break;
        }
        if arg_str == "--lang" {
            return iter.next().and_then(|v| v.to_str().map(ToOwned::to_owned));
        }
        if let Some(val) = arg_str.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Replace a single `{name}` placeholder in a catalog template.
pub fn fmt1(template: &str, key: &str, value: &str) -> String {
    template.replace(&format!("{{{key}}}"), value)
}

/// Replace two `{name}` placeholders in a catalog template.
pub fn fmt2(template: &str, key1: &str, val1: &str, key2: &str, val2: &str) -> String {
    template
        .replace(&format!("{{{key1}}}"), val1)
        .replace(&format!("{{{key2}}}"), val2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        saved: Vec<(&'static str, Option<OsString>)>,
    }

    impl EnvGuard {
        fn capture(keys: &[&'static str]) -> Self {
            let saved = keys.iter().map(|&k| (k, std::env::var_os(k))).collect();
            for &k in keys {
                std::env::remove_var(k);
            }
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, v) in &self.saved {
                match v {
                    Some(val) => std::env::set_var(k, val),
                    None => std::env::remove_var(k),
                }
            }
        }
    }

    fn extract_placeholders(text: &str) -> Vec<String> {
        let mut placeholders = Vec::new();
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'{' {
                if let Some(end) = text[i + 1..].find('}') {
                    let token = &text[i + 1..i + 1 + end];
                    if !token.is_empty()
                        && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        placeholders.push(format!("{{{token}}}"));
                        i += end + 2;
                        continue;
                    }
                }
            }
            i += 1;
        }
        placeholders.sort();
        placeholders
    }

    #[test]
    fn test_all_45_locales_parse() {
        assert_eq!(Locale::ALL.len(), 45);
        for locale in Locale::ALL {
            assert_eq!(Locale::from_tag(locale.code()), Some(locale));
        }

        // Standard POSIX & regional tags
        assert_eq!(Locale::from_tag("C"), Some(Locale::En));
        assert_eq!(Locale::from_tag("POSIX"), Some(Locale::En));
        assert_eq!(Locale::from_tag("en_US.UTF-8"), Some(Locale::En));
        assert_eq!(Locale::from_tag("  de_DE.UTF-8@euro  "), Some(Locale::De));
        assert_eq!(Locale::from_tag("pt_BR.UTF-8"), Some(Locale::PtBr));
        assert_eq!(Locale::from_tag("zh"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh_Hans_CN.UTF-8"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh_TW.UTF-8"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-Hant"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-MO"), Some(Locale::ZhTw));

        // Chinese script precedence over region fallback
        assert_eq!(Locale::from_tag("zh-Hans-HK"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh-Hant-CN"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-HK"), Some(Locale::ZhTw));

        // Singleton-led extension and private-use sections must not pollute core subtags
        assert_eq!(Locale::from_tag("fr-SN-x-ma"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("zh-CN-x-tw"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("ar-EG-u-nu-latn"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("fr-x-ma"), Some(Locale::Fr));
        assert_eq!(Locale::from_tag("ar-t-eg"), Some(Locale::Ar));
        assert_eq!(Locale::from_tag("ar-001"), Some(Locale::Ar));

        // Regional variants must match before primary language fallbacks
        assert_eq!(Locale::from_tag("ar_EG.UTF-8"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar-Arab-EG"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar_MA.UTF-8"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar-Arab-MA"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar_SA.UTF-8"), Some(Locale::Ar));
        assert_eq!(Locale::from_tag("fr_MA.UTF-8"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr-Latn-MA"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr_SN.UTF-8"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr-Latn-SN"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr_CA.UTF-8"), Some(Locale::Fr));

        // Legacy and Phase 2 African/Asian codes
        assert_eq!(Locale::from_tag("iw_IL.UTF-8"), Some(Locale::He));
        assert_eq!(Locale::from_tag("in_ID.UTF-8"), Some(Locale::Id));
        assert_eq!(Locale::from_tag("tl_PH.UTF-8"), Some(Locale::Fil));
        assert_eq!(Locale::from_tag("yo_NG.UTF-8"), Some(Locale::YoNg));
        assert_eq!(Locale::from_tag("ig_NG.UTF-8"), Some(Locale::IgNg));
        assert_eq!(Locale::from_tag("am_ET.UTF-8"), Some(Locale::AmEt));
        assert_eq!(Locale::from_tag("sw_KE.UTF-8"), Some(Locale::SwKe));
        assert_eq!(Locale::from_tag("fa_IR.UTF-8"), Some(Locale::Fa));
        assert_eq!(Locale::from_tag("ur_PK.UTF-8"), Some(Locale::Ur));
        assert_eq!(Locale::from_tag("ms_MY.UTF-8"), Some(Locale::Ms));

        // Empty and unknown tags
        assert_eq!(Locale::from_tag(""), None);
        assert_eq!(Locale::from_tag("   "), None);
        assert_eq!(Locale::from_tag("xx-UNKNOWN"), None);
    }

    #[test]
    fn test_is_rtl_all_locales() {
        let rtl_locales: Vec<Locale> = Locale::ALL
            .iter()
            .copied()
            .filter(|locale| locale.is_rtl())
            .collect();
        let ltr_locales: Vec<Locale> = Locale::ALL
            .iter()
            .copied()
            .filter(|locale| !locale.is_rtl())
            .collect();

        assert_eq!(rtl_locales.len(), 6);
        assert_eq!(ltr_locales.len(), 39);
        assert_eq!(
            rtl_locales,
            vec![
                Locale::Ar,
                Locale::ArEg,
                Locale::ArMa,
                Locale::Fa,
                Locale::He,
                Locale::Ur,
            ]
        );
    }

    #[test]
    fn test_all_catalog_entries_non_empty_and_placeholder_parity() {
        let en = Locale::En.catalog().entries();
        assert_eq!(en.len(), 67);

        for locale in Locale::ALL {
            let entries = locale.catalog().entries();
            assert_eq!(entries.len(), en.len());
            for ((en_key, en_val), (loc_key, loc_val)) in en.iter().zip(entries.iter()) {
                assert_eq!(en_key, loc_key);
                assert!(
                    !loc_val.trim().is_empty(),
                    "Empty translation for {loc_key} in {}",
                    locale.code()
                );
                assert_eq!(
                    extract_placeholders(loc_val),
                    extract_placeholders(en_val),
                    "Placeholder mismatch for {loc_key} in {}: got {loc_val:?}",
                    locale.code()
                );
            }
        }
    }

    #[test]
    fn test_default_english_strings_backward_compatible() {
        let en = Locale::En.catalog();
        assert_eq!(en.cli_about, "CLI tool for creating conventional commits");
        assert_eq!(en.prompt_select_type, "Select commit type");
        assert_eq!(en.msg_commit_valid, "Commit message is valid.");
        assert_eq!(en.err_not_git_repo, "Not in a git repository");
    }

    #[test]
    fn test_detect_and_resolve_env_precedence() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _guard = EnvGuard::capture(&["LC_ALL", "LC_MESSAGES", "LANG"]);

        // No env vars set -> fallback to English
        assert_eq!(Locale::detect(), Locale::En);
        assert_eq!(Locale::resolve(None), Locale::En);

        // LANG only
        std::env::set_var("LANG", "ja_JP.UTF-8");
        assert_eq!(Locale::detect(), Locale::Ja);
        assert_eq!(Locale::resolve(None), Locale::Ja);

        // Empty LC_MESSAGES is skipped in favor of LANG
        std::env::set_var("LC_MESSAGES", "   ");
        assert_eq!(Locale::detect(), Locale::Ja);

        // LC_MESSAGES overrides LANG
        std::env::set_var("LC_MESSAGES", "fr_SN.UTF-8");
        assert_eq!(Locale::detect(), Locale::FrSn);

        // LC_ALL overrides LC_MESSAGES and LANG
        std::env::set_var("LC_ALL", "de_DE.UTF-8");
        assert_eq!(Locale::detect(), Locale::De);

        // Unknown value in highest-priority var falls back to English
        std::env::set_var("LC_ALL", "zz_ZZ.UTF-8");
        assert_eq!(Locale::detect(), Locale::En);

        // Explicit tag overrides environment; unknown explicit tag falls back to English
        std::env::set_var("LC_ALL", "de_DE.UTF-8");
        assert_eq!(Locale::resolve(Some("pt-BR")), Locale::PtBr);
        assert_eq!(Locale::resolve(Some("xx-UNKNOWN")), Locale::En);
    }

    #[test]
    fn test_fmt1_and_fmt2() {
        assert_eq!(
            fmt1("Error: {error}", "error", "config missing"),
            "Error: config missing"
        );
        assert_eq!(fmt1("{x} + {x}", "x", "1"), "1 + 1");
        assert_eq!(
            fmt2(
                "Using template: {name} - {description}",
                "name",
                "feature",
                "description",
                "Standard feature"
            ),
            "Using template: feature - Standard feature"
        );
    }

    #[test]
    fn test_extract_cli_lang() {
        let args = vec![
            "commitkit".to_string(),
            "--lang".to_string(),
            "ja".to_string(),
            "--help".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args), Some("ja".to_string()));

        let args_eq = vec!["commitkit".to_string(), "--lang=pt-BR".to_string()];
        assert_eq!(extract_cli_lang(&args_eq), Some("pt-BR".to_string()));

        let args_none = vec!["commitkit".to_string(), "--dry-run".to_string()];
        assert_eq!(extract_cli_lang(&args_none), None);

        let args_after_double_dash = vec![
            "commitkit".to_string(),
            "--".to_string(),
            "--lang".to_string(),
            "de".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args_after_double_dash), None);

        let args_trailing_flag = vec!["commitkit".to_string(), "--lang".to_string()];
        assert_eq!(extract_cli_lang(&args_trailing_flag), None);
    }

    #[test]
    fn test_extract_cli_lang_os() {
        let to_os =
            |slice: &[&str]| -> Vec<OsString> { slice.iter().map(OsString::from).collect() };

        assert_eq!(
            extract_cli_lang_os(&to_os(&["commitkit", "--lang", "ja", "--help"])),
            Some("ja".to_string())
        );
        assert_eq!(
            extract_cli_lang_os(&to_os(&["commitkit", "--lang=pt-BR"])),
            Some("pt-BR".to_string())
        );
        assert_eq!(
            extract_cli_lang_os(&to_os(&["commitkit", "--", "--lang", "de"])),
            None
        );
        assert_eq!(
            extract_cli_lang_os(&to_os(&["commitkit", "--", "--lang=de"])),
            None
        );
        assert_eq!(extract_cli_lang_os(&to_os(&["commitkit", "--lang"])), None);
        assert_eq!(
            extract_cli_lang_os(&to_os(&["commitkit", "--dry-run"])),
            None
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_cli_lang_os_non_utf8() {
        use std::os::unix::ffi::OsStringExt;

        let non_utf8_path = OsString::from_vec(vec![b'/', b't', b'm', b'p', b'/', 0xff, 0xfe]);
        let args_before = vec![
            OsString::from("commitkit"),
            OsString::from("--config"),
            non_utf8_path.clone(),
            OsString::from("--lang"),
            OsString::from("de"),
        ];
        assert_eq!(extract_cli_lang_os(&args_before), Some("de".to_string()));

        let args_non_utf8_val = vec![
            OsString::from("commitkit"),
            OsString::from("--lang"),
            non_utf8_path,
        ];
        assert_eq!(extract_cli_lang_os(&args_non_utf8_val), None);

        let mut eq_non_utf8 = b"--lang=".to_vec();
        eq_non_utf8.extend_from_slice(&[0xff, 0xfe]);
        let args_eq_non_utf8 = vec![OsString::from("commitkit"), OsString::from_vec(eq_non_utf8)];
        assert_eq!(extract_cli_lang_os(&args_eq_non_utf8), None);
    }
}
