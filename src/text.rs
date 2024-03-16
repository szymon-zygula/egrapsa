pub trait TextNode {
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

pub struct TextTree {
    pub name: Option<String>,
    pub subtexts: Vec<Box<dyn TextNode>>,
}

impl TextNode for TextTree {
    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
