use crate::text::TextTree;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetTextError {
    #[error("there was a problem with connecting to the text source")]
    ConnectionError,
    #[error("could not encode text downloaded from the text source as a string")]
    EncodingError,
    #[error("the data downloaded from the text source could not be parsed")]
    ParseError,
}

type GetTextResult = Result<TextTree, GetTextError>;

pub trait TextSource {
    fn get_text(&self, id: &str) -> GetTextResult;
}

pub mod gutenberg;
pub mod gutenberg_de;
pub mod scaife;
