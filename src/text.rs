pub trait TextNode: std::fmt::Debug {
    fn name(&self) -> Option<&String>;
    fn to_string(&self) -> String;
    fn remove_new_lines(&self) -> Box<dyn TextNode>;

    // Visitor functions for formatters
    fn format_for_latex(&self) -> String;
}

impl TextNode for String {
    fn name(&self) -> Option<&String> {
        None
    }

    fn to_string(&self) -> String {
        self.clone()
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        Box::new(self.replace('\n', " "))
    }

    fn format_for_latex(&self) -> String {
        self.clone()
    }
}

impl TextNode for &str {
    fn name(&self) -> Option<&String> {
        None
    }

    fn to_string(&self) -> String {
        String::from(*self)
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        Box::new(self.replace('\n', " "))
    }

    fn format_for_latex(&self) -> String {
        TextNode::to_string(self)
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

    fn to_string(&self) -> String {
        self.subtexts
            .iter()
            .map(|subtext| subtext.to_string())
            .fold(String::new(), |a, b| [&a, " ", &b].concat())
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        Box::new(Self {
            name: self.name.clone(),
            subtexts: self
                .subtexts
                .iter()
                .map(|subtext| subtext.remove_new_lines())
                .collect(),
        })
    }

    fn format_for_latex(&self) -> String {
        self.subtexts
            .iter()
            .map(|subtext| subtext.format_for_latex())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
pub struct Footnote(pub String);

impl TextNode for Footnote {
    fn name(&self) -> Option<&String> {
        None
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        Box::new(Self(self.0.replace('\n', " ")))
    }

    fn format_for_latex(&self) -> String {
        format!("\\footnote{{{}}}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParagraphNumber(pub u32);

impl TextNode for ParagraphNumber {
    fn name(&self) -> Option<&String> {
        None
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        Box::new(*self)
    }

    fn format_for_latex(&self) -> String {
        let mut text = String::from("\n");
        text.push_str(r"\centering{\footnotesize\color{gray} p.");
        text.push_str(&self.0.to_string());
        text.push_str("}\n");
        text
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineNumber(pub u32);

impl TextNode for LineNumber {
    fn name(&self) -> Option<&String> {
        None
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        Box::new(*self)
    }

    fn format_for_latex(&self) -> String {
        String::new()
    }
}
