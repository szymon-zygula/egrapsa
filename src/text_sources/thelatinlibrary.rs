use super::{GetTextResult, TextSource};

struct TheLatinLibrary {}

impl TextSource for TheLatinLibrary {
    fn get_text(&self, _id: &str) -> GetTextResult {
        todo!()
    }
}
