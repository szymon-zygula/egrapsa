use crate::formatters::{latex, Language, TextFormatter, Work};
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
    pub margin_notes: bool,
    pub language: Language,
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
        formatter.set_margin_notes(config.margin_notes);
        formatter.set_language(config.language);

        formatter
    }

    pub fn take_work_infos(self) -> Vec<WorkInfo> {
        self.work_infos
    }
}
