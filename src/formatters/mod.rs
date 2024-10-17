use crate::text::TextParent;
use serde::{Deserialize, Serialize};

pub struct Work {
    pub title: String,
    // It's popular to have bilingual work names in Greek books
    pub alt_title: Option<String>,
    pub text: TextParent,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Language {
    Latin,
    Greek,
}

impl Default for Language {
    fn default() -> Self {
        Self::Latin
    }
}

pub trait TextFormatter {
    fn set_title(&mut self, title: Option<String>);
    fn set_author(&mut self, author: Option<String>);
    fn set_catchwords(&mut self, catchwords: bool);
    fn set_margin_notes(&mut self, margin_notes: bool);
    fn set_footnotes(&mut self, footnotes: bool);
    fn set_language(&mut self, language: Language);
    fn add_work(&mut self, work: Work);
    fn format(&self) -> String;
}

pub mod latex;
