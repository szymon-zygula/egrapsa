use crate::text::TextTree;

pub trait TextFormatter {
    fn format(text: &TextTree) -> String;
}

pub mod latex;
