use crate::text::TextParent;

pub trait TextFormatter {
    fn title(self, title: Option<String>) -> Self;
    fn author(self, author: Option<String>) -> Self;
    fn format(&self, text: &TextParent) -> String;
}

pub mod latex;
