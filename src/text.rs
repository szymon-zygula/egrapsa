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
    DialogueEntry,
    Regularized,
    Date,
    Apparatus,
    Lemma,
    Highlight,
    Choice,
    Abbreviated,
    Expanded,
    Expandable,
    Description,
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
            TextNodeKind::Regularized => {}
            TextNodeKind::Apparatus => {}
            TextNodeKind::Date => {}
            TextNodeKind::Speaker => {
                let mut text = String::from(r"\vspace{6pt}\textbf{");
                text.push_str(&formatted);
                text.push_str("}· ");
                formatted = text;
            }
            TextNodeKind::DialogueEntry => {}
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
            TextNodeKind::Lemma => {}
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
            TextNodeKind::Highlight => {}
            TextNodeKind::Deleted => {}
            TextNodeKind::Corrected => {}
            TextNodeKind::Label => {
                let mut text = String::from(r"\textbf{");
                text.push_str(&formatted);
                text.push_str("} ");
                formatted = text;
            }
            TextNodeKind::Quote => {}
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
            TextNodeKind::Choice => {}
            TextNodeKind::Abbreviated => formatted = String::new(), // Always use expanded version
            TextNodeKind::Expanded => {}
            TextNodeKind::Expandable => {}
            TextNodeKind::Description => {}
        }

        formatted = format!(" {} ", formatted);
        fix_text(&formatted)
    }
}

#[derive(Debug)]
pub struct Footnote(pub String);

impl TextNode for Footnote {
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self) -> String {
        format!("\\footnote{{{}}} ", self.0.format_for_latex())
    }
}

#[derive(Debug, Clone)]
pub struct ParagraphNumber(pub String);

impl TextNode for ParagraphNumber {
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self) -> String {
        let mut text = String::from(r"\alignedmarginpar{");
        text.push_str(&self.0);
        text.push_str("}");
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
        let mut text = String::from(r"\alignedmarginpar{");
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
            let mut text = String::from(r"\alignedmarginpar{");
            text.push_str(number);
            text.push_str("}");
            text
        } else {
            String::new()
        }
    }
}

#[derive(Debug)]
pub struct Highlight {
    pub rend: String,
    pub text: Box<dyn TextNode>,
}

impl TextNode for Highlight {
    fn to_string(&self) -> String {
        self.text.to_string()
    }

    fn format_for_latex(&self) -> String {
        let inner = self.text.format_for_latex();
        let mark = match self.rend.as_str() {
            "italics" => "textit",
            _ => panic!("Unknown <hi> rend type ({})", self.rend),
        };

        format!(" \\{}{{{}}} ", mark, inner)
    }
}

// "lost" does not look good when all other footnotes are in Latin.
// On the other hand, when footnotes are in English, "lacuna" is still acceptable,
// although somewhat over-the-top (so perfect for this project)
fn translate_gap_reason(reason: &str) -> &str {
    match reason {
        "lost" => "lacuna",
        x => x,
    }
}

#[derive(Debug, Clone)]
pub struct Gap {
    pub reason: String,
    pub rend: Option<String>,
}

impl TextNode for Gap {
    fn to_string(&self) -> String {
        format!(
            "{} [{}]",
            self.rend.as_ref().map(|x| x.as_str()).unwrap_or("[\\dots]"),
            translate_gap_reason(&self.reason)
        )
    }

    fn format_for_latex(&self) -> String {
        format!(
            "{}\\footnote{{{}}} ",
            self.rend.as_ref().map(|x| x.as_str()).unwrap_or("[\\dots]"),
            translate_gap_reason(&self.reason)
        )
    }
}

fn fix_punctuation(text: &mut String, p: &str) {
    // A quick way to normalize spaces
    *text = text
        .replace(p, &format!("{} ", p))
        .replace(&format!(" {}", p), p)
        .replace(&format!("{}   ", p), &format!("{} ", p))
        .replace(&format!("{}  ", p), &format!("{} ", p));
}

pub fn fix_text(text: &str) -> String {
    let mut text = text
        .replace("&gt;", "")
        .replace("&lt;", "") // Remove junk
        .replace(" — ", "—")
        .replace("— ", "—")
        .replace(" —", "—")
        .replace("—", "---");

    fix_punctuation(&mut text, ",");
    fix_punctuation(&mut text, ".");
    fix_punctuation(&mut text, "?");
    fix_punctuation(&mut text, ";");
    fix_punctuation(&mut text, ";"); // Greek question mark
    fix_punctuation(&mut text, ":");
    fix_punctuation(&mut text, "·");

    text
}
