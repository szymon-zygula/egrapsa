use crate::text::TextParent;

pub struct Work {
    pub title: String,
    pub text: TextParent
}

pub trait TextFormatter {
    fn title(self, title: Option<String>) -> Self;
    fn author(self, author: Option<String>) -> Self;
    fn add_work(self, work: Work) -> Self;
    fn format(&self) -> String;
}

pub mod latex;
