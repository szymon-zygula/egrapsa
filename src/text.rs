pub trait TextNode: std::fmt::Debug {
    fn to_string(&self) -> String;

    // Visitor functions for formatters
    fn format_for_latex(&self) -> String;
}

impl TextNode for String {
    fn to_string(&self) -> String {
        self.clone()
    }

    fn format_for_latex(&self) -> String {
        self.clone().replace('#', r"\#")
    }
}

impl TextNode for &str {
    fn to_string(&self) -> String {
        String::from(*self)
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
    Sic,
    Italics,
    Line,
    Simple,
    Corrected,
    Symbol,
    Speaker,
    DialogueEntry
}

#[derive(Debug)]
pub struct TextParent {
    pub name: Option<Box<dyn TextNode>>,
    pub kind: TextNodeKind,
    pub subtexts: Vec<Box<dyn TextNode>>,
}

impl TextNode for TextParent {
    fn to_string(&self) -> String {
        self.subtexts
            .iter()
            .map(|subtext| subtext.to_string())
            .fold(String::new(), |a, b| [&a, " ", &b].concat())
    }

    fn format_for_latex(&self) -> String {
        let mut formatted: String = self
            .subtexts
            .iter()
            .map(|subtext| subtext.format_for_latex())
            .filter(|subtext| !subtext.is_empty())
            .collect();

        match self.kind {
            TextNodeKind::Sic => {
                // Sic seems to be ignored by original perseus reader, but is rendered by Scaife.
                // Example occurence in a text:
                // <sic><corr>ἑαυτόν·</corr></sic><corr>ἑαυτόν·</corr>
                // This seems nonsensical. We'll ignore <sic> for now.
                formatted = String::new()
            }
            TextNodeKind::Speaker => {
                let mut text = String::from(r"\textbf{");
                text.push_str(&formatted);
                text.push_str("}· ");
                formatted = text;
            }
            TextNodeKind::DialogueEntry => {
            }
            TextNodeKind::Symbol => {
                let mut text = String::from(r"\textit{");
                text.push_str(&formatted);
                text.push_str("}");
                formatted = text;
            }
            TextNodeKind::Book => {
                // Title etc. are taken from input parameters
            }
            TextNodeKind::Chapter => {}
            TextNodeKind::Section => {
                let name = self
                    .name
                    .as_ref()
                    .map(|s| s.format_for_latex())
                    .unwrap_or_default();
                let mut text = String::from(r"\section*{");
                text.push_str(&name);
                text.push('}');
                text.push_str(&formatted);
                formatted = text;
            }
            TextNodeKind::SubSection => {
                let name = self
                    .name
                    .as_ref()
                    .map(|s| s.format_for_latex())
                    .unwrap_or_default();
                let mut text = String::from(r"\subsection*{");
                text.push_str(&name);
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
            TextNodeKind::Corrected => {
                formatted = format!(" {} ", formatted);
            }
            TextNodeKind::Label => {
                let mut text = String::from(r"\textbf{");
                text.push_str(&formatted);
                text.push_str("} ");
                formatted = text;
            }
            TextNodeKind::Quote => {
                formatted = format!(" {} ", formatted);
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
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self) -> String {
        format!("\\footnote{{{}}}", self.0.format_for_latex())
    }
}

#[derive(Debug, Clone)]
pub struct ParagraphNumber(pub String);

impl TextNode for ParagraphNumber {
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self) -> String {
        let mut text = String::from("\n");
        text.push_str(r"\alignedmarginpar{\footnotesize\color{gray} p.");
        text.push_str(&self.0);
        text.push_str("}\n");
        text
    }
}

#[derive(Debug, Clone)]
pub struct LineNumber(pub String);

impl TextNode for LineNumber {
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self) -> String {
        let mut text = String::from(r"\alignedmarginpar{\footnotesize\color{gray}");
        text.push_str(&self.0);
        text.push_str("}");
        text
    }
}

#[derive(Debug, Clone)]
pub struct Milestone {
    pub unit: String,
    pub number: Option<String>,
    pub ed: Option<String>,
    pub resp: Option<String>,
}

impl TextNode for Milestone {
    fn to_string(&self) -> String {
        if let Some(number) = &self.number {
            format!("({})", number)
        } else {
            String::new()
        }
    }

    fn format_for_latex(&self) -> String {
        if self.unit == "page" || self.unit == "speech" {
            return String::new();
        }

        if let Some(number) = &self.number {
            let mut text = String::from(r"\alignedmarginpar{\footnotesize\color{gray}");
            text.push_str(number);
            text.push_str("}");
            text
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gap {
    pub reason: String,
    pub rend: String,
}

impl TextNode for Gap {
    fn to_string(&self) -> String {
        format!("{} [{}]", self.rend, self.reason)
    }

    fn format_for_latex(&self) -> String {
        format!("{}\\footnote{{{}}}", self.rend, self.reason)
    }
}
