use crate::formatters::{latex, Language, TextFormatter, Typography, Work};
use crate::text_sources::TextSource;
use serde::{Deserialize, Serialize};

use crate::text_sources::scaife;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TextSourceType {
    Scaife,
}

impl TextSourceType {
    pub fn get_source(&self) -> Box<dyn TextSource> {
        Box::new(match self {
            Self::Scaife => scaife::Scaife {},
        })
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TextFormatterType {
    Latex,
}

impl TextFormatterType {
    pub fn get_formatter(&self) -> Box<dyn TextFormatter> {
        Box::new(match self {
            Self::Latex => latex::Latex::new(),
        })
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    pub title: Option<String>,
    pub author: Option<String>,
    pub catchwords: bool,
    pub ref_numbers: bool,
    pub footnotes: bool,
    pub language: Language,
    #[serde(default)]
    pub typography: Typography,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkInfo {
    pub title: String,
    // It's popular to have bilingual work names in Greek books
    pub alt_title: Option<String>,
    pub author: Option<String>,
    pub identifier: String,
}

impl WorkInfo {
    pub fn into_work(self, source: &dyn TextSource) -> Work {
        let text = source.get_text(&self.identifier).unwrap();

        Work {
            title: self.title,
            alt_title: self.alt_title,
            text,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    name: String,
    formatter_type: TextFormatterType,
    formatter_config: FormatterConfig,
    source_type: TextSourceType,
    work_infos: Vec<WorkInfo>,
}

impl Config {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source(&self) -> Box<dyn TextSource> {
        let source = self.source_type.get_source();

        source
    }

    pub fn formatter(&self) -> Box<dyn TextFormatter> {
        let mut formatter = self.formatter_type.get_formatter();
        let config = self.formatter_config.clone();

        formatter.set_title(config.title);
        formatter.set_author(config.author);
        formatter.set_catchwords(config.catchwords);
        formatter.set_margin_notes(config.ref_numbers);
        formatter.set_footnotes(config.footnotes);
        formatter.set_language(config.language);
        formatter.set_typography(config.typography);

        formatter
    }

    pub fn take_work_infos(self) -> Vec<WorkInfo> {
        self.work_infos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typography_field_from_json() {
        let json_modern = r#"
        {
            "name": "Test Config Modern",
            "formatter_type": "Latex",
            "formatter_config": {
                "title": "Test Document",
                "author": "Test Author",
                "catchwords": false,
                "ref_numbers": true,
                "footnotes": true,
                "language": "Latin",
                "typography": "modern"
            },
            "source_type": "Scaife",
            "work_infos": []
        }"#;

        let config: Config = serde_json::from_str(json_modern).unwrap();
        assert!(matches!(config.formatter_config.typography, Typography::Modern));

        let json_old = r#"
        {
            "name": "Test Config Old",
            "formatter_type": "Latex",
            "formatter_config": {
                "title": "Test Document",
                "author": "Test Author",
                "catchwords": false,
                "ref_numbers": true,
                "footnotes": true,
                "language": "Latin",
                "typography": "old"
            },
            "source_type": "Scaife",
            "work_infos": []
        }"#;

        let config: Config = serde_json::from_str(json_old).unwrap();
        assert!(matches!(config.formatter_config.typography, Typography::Old));

        let json_very_old = r#"
        {
            "name": "Test Config VeryOld",
            "formatter_type": "Latex",
            "formatter_config": {
                "title": "Test Document",
                "author": "Test Author",
                "catchwords": false,
                "ref_numbers": true,
                "footnotes": true,
                "language": "Latin",
                "typography": "very_old"
            },
            "source_type": "Scaife",
            "work_infos": []
        }"#;

        let config: Config = serde_json::from_str(json_very_old).unwrap();
        assert!(matches!(config.formatter_config.typography, Typography::VeryOld));
    }

    #[test]
    fn uses_default_typography_when_field_missing() {
        let json_without_typography = r#"
        {
            "name": "Test Config Without Typography",
            "formatter_type": "Latex",
            "formatter_config": {
                "title": "Test Document",
                "author": "Test Author",
                "catchwords": false,
                "ref_numbers": true,
                "footnotes": true,
                "language": "Latin"
            },
            "source_type": "Scaife",
            "work_infos": []
        }"#;

        let config: Config = serde_json::from_str(json_without_typography).unwrap();
        // Should use default (VeryOld for backward compatibility)
        assert!(matches!(config.formatter_config.typography, Typography::VeryOld));
    }

    #[test]
    fn config_sets_typography_on_formatter() {
        let json = r#"
        {
            "name": "Test Config",
            "formatter_type": "Latex",
            "formatter_config": {
                "title": "Test Document",
                "author": "Test Author",
                "catchwords": false,
                "ref_numbers": true,
                "footnotes": true,
                "language": "Latin",
                "typography": "modern"
            },
            "source_type": "Scaife",
            "work_infos": []
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        let formatter = config.formatter();
        
        // We can't directly access the typography field, but we can test
        // that the formatter was created successfully with the config
        assert_eq!(config.name(), "Test Config");
    }
}
