use crate::text::TextParent;

pub struct Work {
    pub title: String,
    // It's popular to have bilingual work names in Greek books
    pub alt_title: Option<String>,
    pub text: TextParent
}

pub trait TextFormatter {
    fn title(self, title: Option<String>) -> Self;
    fn author(self, author: Option<String>) -> Self;
    fn add_work(self, work: Work) -> Self;
    fn catchwords(self, catchwords: bool) -> Self;
    fn format(&self) -> String;
}

pub mod latex;
