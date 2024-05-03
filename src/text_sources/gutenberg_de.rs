use super::{GetTextResult, TextSource};

struct GutenbergDe {}

impl TextSource for GutenbergDe {
    fn get_text(&self, _id: &str) -> GetTextResult {
        todo!()
    }
}
