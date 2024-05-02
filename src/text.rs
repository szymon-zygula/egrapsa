pub trait TextNode: std::fmt::Debug {
    fn name(&self) -> Option<&String>;
}

impl TextNode for String {
    fn name(&self) -> Option<&String> {
        None
    }
}

impl TextNode for &str {
    fn name(&self) -> Option<&String> {
        None
    }
}

#[derive(Debug)]
pub struct TextTree {
    pub name: Option<String>,
    pub subtexts: Vec<Box<dyn TextNode>>,
}

impl TextNode for TextTree {
    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

#[derive(Debug)]
pub struct Footnote(pub String);

impl TextNode for Footnote {
    fn name(&self) -> Option<&String> {
        None
    }
}

#[derive(Debug)]
pub struct ParagraphNumber(pub u32);

impl TextNode for ParagraphNumber {
    fn name(&self) -> Option<&String> {
        None
    }
}

#[derive(Debug)]
pub struct LineNumber(pub u32);

impl TextNode for LineNumber {
    fn name(&self) -> Option<&String> {
        None
    }
}
