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
        self.clone().replace('#', r"\#")
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
        TextNode::to_string(self).format_for_latex()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextNodeKind {
    Book,
    Chapter,
    Section,
    SubSection,
    Subsection,
    Paragraph,
    Note,
    Deleted,
    Label,
    Quote,
    BlockQuote,
    Italics,
    Line,
    Simple,
}

#[derive(Debug)]
pub struct TextParent {
    pub name: Option<String>,
    pub kind: TextNodeKind,
    pub subtexts: Vec<Box<dyn TextNode>>,
}

impl TextNode for TextParent {
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
            kind: self.kind,
            subtexts: self
                .subtexts
                .iter()
                .map(|subtext| subtext.remove_new_lines())
                .collect(),
        })
    }

    fn format_for_latex(&self) -> String {
        let mut formatted: String = self
            .subtexts
            .iter()
            .map(|subtext| subtext.format_for_latex())
            .collect();

        match self.kind {
            TextNodeKind::Book => {
                // let mut text = String::from("\\maketitle\n");
                // text.push('}');
                // text.push_str(&formatted);
                // formatted = text;
            }
            TextNodeKind::Chapter => {}
            TextNodeKind::Section => {
                let name = self.name.as_ref().map(|s| s.as_str()).unwrap_or("");
                let mut text = String::from(r"\section{");
                text.push_str(name);
                text.push('}');
                text.push_str(&formatted);
                formatted = text;
            }
            TextNodeKind::SubSection => {
                let name = self.name.as_ref().map(|s| s.as_str()).unwrap_or("");
                let mut text = String::from(r"\subsection{");
                text.push_str(name);
                text.push('}');
                text.push_str(&formatted);
                formatted = text;
            }
            TextNodeKind::Subsection => {}
            TextNodeKind::Paragraph => {
                formatted.push_str("\n\n");
            }
            TextNodeKind::Note => {}
            TextNodeKind::Deleted => {}
            TextNodeKind::Label => {
                let mut text = String::from(r"\textbf{");
                text.push_str(&formatted);
                text.push_str("} ");
                formatted = text;
            }
            TextNodeKind::Quote => {
                let mut text = String::from(r"\say{");
                text.push_str(&formatted);
                text.push_str("}");
                formatted = text;
            }
            TextNodeKind::BlockQuote => {
                let mut text = String::from(r"\begin{displayquote}");
                text.push_str(&formatted);
                text.push_str(r"\end{displayquote}");
                formatted = text;
            }
            TextNodeKind::Italics => {
                let mut text = String::from(r"\textit{");
                text.push_str(&formatted);
                text.push_str("}");
                formatted = text;
            }
            TextNodeKind::Line => {
                formatted.push_str("\n\\\\");
            }
            TextNodeKind::Simple => {}
        }

        formatted
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
        format!("\\footnote{{{}}}", self.0.format_for_latex())
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
        text.push_str(r"\alignedmarginpar{\footnotesize\color{gray} p.");
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
        let mut text = String::from("\n");
        text.push_str(r"\alignedmarginpar{\footnotesize\color{gray}");
        text.push_str(&self.0.to_string());
        text.push_str("}\n");
        text
    }
}

#[derive(Debug, Clone)]
pub struct Gap(pub String);

impl TextNode for Gap {
    fn name(&self) -> Option<&String> {
        None
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn remove_new_lines(&self) -> Box<dyn TextNode> {
        self.0.remove_new_lines()
    }

    fn format_for_latex(&self) -> String {
        self.0.format_for_latex()
    }
}
