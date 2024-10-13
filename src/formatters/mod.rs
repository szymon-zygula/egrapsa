use crate::text::TextParent;

pub struct Work {
    pub title: String,
    // It's popular to have bilingual work names in Greek books
    pub alt_title: Option<String>,
    pub text: TextParent,
}

pub trait TextFormatter {
    fn set_title(&mut self, title: Option<String>);
    fn set_author(&mut self, author: Option<String>);
    fn set_catchwords(&mut self, catchwords: bool);
    fn add_work(&mut self, work: Work);
    fn format(&self) -> String;
}

pub mod latex;
