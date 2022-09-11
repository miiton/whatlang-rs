use std::fmt;
use std::str::FromStr;

use super::lang_mapping;
use crate::error::ParseError;
use crate::Lang;

#[cfg(feature = "enum-map")]
use enum_map::Enum;

/// Represents a writing system (Latin, Cyrillic, Arabic, etc).
#[cfg_attr(feature = "enum-map", derive(Enum))]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Script {
    // Keep this in alphabetic order (for C bindings)
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Hiragana,
    Kanji,
    Kannada,
    Katakana,
    Khmer,
    Latin,
    Malayalam,
    Mandarin,
    Myanmar,
    Oriya,
    SimplifiedChinese,
    Sinhala,
    Tamil,
    Telugu,
    Thai,
    TraditionalChinese,
}

// Array of all existing Script values.
const VALUES: [Script; 28] = [
    Script::Arabic,
    Script::Armenian,
    Script::Bengali,
    Script::Cyrillic,
    Script::Devanagari,
    Script::Ethiopic,
    Script::Georgian,
    Script::Greek,
    Script::Gujarati,
    Script::Gurmukhi,
    Script::Hangul,
    Script::Hebrew,
    Script::Hiragana,
    Script::Kanji,
    Script::Kannada,
    Script::Katakana,
    Script::Khmer,
    Script::Latin,
    Script::Malayalam,
    Script::Mandarin,
    Script::Myanmar,
    Script::Oriya,
    Script::SimplifiedChinese,
    Script::Sinhala,
    Script::Tamil,
    Script::Telugu,
    Script::Thai,
    Script::TraditionalChinese,
];

impl Script {
    /// Get all existing scripts.
    ///
    /// # Example
    /// ```
    /// use whatlang::Script;
    /// for script in Script::all() {
    ///     println!("{}", script);
    /// }
    /// ```
    pub fn all() -> &'static [Script] {
        &VALUES
    }

    pub fn name(&self) -> &str {
        match *self {
            Script::Latin => "Latin",
            Script::Cyrillic => "Cyrillic",
            Script::Arabic => "Arabic",
            Script::Devanagari => "Devanagari",
            Script::Hiragana => "Hiragana",
            Script::Kanji => "Kanji",
            Script::Katakana => "Katakana",
            Script::Ethiopic => "Ethiopic",
            Script::Hebrew => "Hebrew",
            Script::Bengali => "Bengali",
            Script::Georgian => "Georgian",
            Script::Mandarin => "Mandarin",
            Script::Hangul => "Hangul",
            Script::Greek => "Greek",
            Script::Kannada => "Kannada",
            Script::Tamil => "Tamil",
            Script::Thai => "Thai",
            Script::Gujarati => "Gujarati",
            Script::Gurmukhi => "Gurmukhi",
            Script::Telugu => "Telugu",
            Script::Malayalam => "Malayalam",
            Script::Oriya => "Oriya",
            Script::Myanmar => "Myanmar",
            Script::Sinhala => "Sinhala",
            Script::SimplifiedChinese => "SimplifiedChinese",
            Script::Khmer => "Khmer",
            Script::Armenian => "Armenian",
            Script::TraditionalChinese => "TraditionalChinese",
        }
    }

    pub fn langs(&self) -> &[Lang] {
        lang_mapping::script_langs(*self)
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Script {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "latin" => Ok(Script::Latin),
            "cyrillic" => Ok(Script::Cyrillic),
            "arabic" => Ok(Script::Arabic),
            "devanagari" => Ok(Script::Devanagari),
            "hiragana" => Ok(Script::Hiragana),
            "kanji" => Ok(Script::Kanji),
            "katakana" => Ok(Script::Katakana),
            "ethiopic" => Ok(Script::Ethiopic),
            "hebrew" => Ok(Script::Hebrew),
            "bengali" => Ok(Script::Bengali),
            "georgian" => Ok(Script::Georgian),
            "mandarin" => Ok(Script::Mandarin),
            "simplifiedchinese" => Ok(Script::SimplifiedChinese),
            "traditionalchinese" => Ok(Script::TraditionalChinese),
            "hangul" => Ok(Script::Hangul),
            "greek" => Ok(Script::Greek),
            "kannada" => Ok(Script::Kannada),
            "tamil" => Ok(Script::Tamil),
            "thai" => Ok(Script::Thai),
            "gujarati" => Ok(Script::Gujarati),
            "gurmukhi" => Ok(Script::Gurmukhi),
            "telugu" => Ok(Script::Telugu),
            "malayalam" => Ok(Script::Malayalam),
            "oriya" => Ok(Script::Oriya),
            "myanmar" => Ok(Script::Myanmar),
            "sinhala" => Ok(Script::Sinhala),
            "khmer" => Ok(Script::Khmer),
            "armenian" => Ok(Script::Armenian),
            _ => Err(ParseError::Script(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(Script::all().len(), 28);
        let all = Script::all();
        assert!(all.contains(&Script::Cyrillic));
        assert!(all.contains(&Script::Arabic));
        assert!(all.contains(&Script::Latin));
    }

    #[test]
    fn test_from_str() {
        for &script in Script::all() {
            let s = script.name();
            assert_eq!(s.parse::<Script>().unwrap(), script);
            assert_eq!(s.to_lowercase().parse::<Script>().unwrap(), script);
            assert_eq!(s.to_uppercase().parse::<Script>().unwrap(), script);
        }

        let result = "foobar".parse::<Script>();
        assert!(matches!(result, Err(ParseError::Script(_))));
    }

    #[test]
    fn test_langs() {
        // Vec of all langs obtained with script.langs()
        let script_langs: Vec<Lang> = Script::all()
            .iter()
            .map(|script| script.langs())
            .flatten()
            .copied()
            .collect();

        // Ensure all langs belong at least to one script
        for lang in Lang::all() {
            assert!(script_langs.contains(&lang));
        }
    }
}
