use super::{GetTextResult, TextSource};

struct Gutenberg {}

impl TextSource for Gutenberg {
    fn get_text(&self, id: &str) -> GetTextResult {
        todo!()
    }
}
