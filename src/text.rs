use crate::config::FormatterConfig;
use std::borrow::Cow;

pub trait TextNode: std::fmt::Debug {
    fn to_string(&self) -> String;

    // Visitor functions for formatters
    fn format_for_latex(&self, config: &FormatterConfig) -> String;
}

impl TextNode for String {
    fn to_string(&self) -> String {
        self.clone()
    }

    fn format_for_latex(&self, _config: &FormatterConfig) -> String {
        normalize_text(self.clone())
    }
}

impl TextNode for &str {
    fn to_string(&self) -> String {
        String::from(*self)
    }

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        TextNode::to_string(self).format_for_latex(config)
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

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        let mut formatted: String = self
            .subtexts
            .iter()
            .map(|subtext| subtext.format_for_latex(config))
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
                let mut text = String::from(r"\vspace{6pt}\Needspace{2\baselineskip}\textbf{");
                text.push_str(&formatted);
                text.push_str(r"}·\\");
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
                    .map(|s| s.format_for_latex(config))
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
                    .map(|s| s.format_for_latex(config))
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

        formatted = replace_et_ampersand(formatted);
        fix_text(formatted)
    }
}

fn ensure_dot(str: &str) -> Cow<str> {
    if str.ends_with('.') || str.ends_with(". ") {
        Cow::Borrowed(str)
    } else {
        Cow::Owned(String::from(str) + ".")
    }
}

#[derive(Debug)]
pub struct Footnote(pub String);

impl TextNode for Footnote {
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        if config.footnotes {
            format!(
                "\\footnote{{{}}} ",
                ensure_dot(&self.0.format_for_latex(config))
            )
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParagraphNumber(pub String);

impl TextNode for ParagraphNumber {
    fn to_string(&self) -> String {
        self.0.clone()
    }

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        let mut text = String::from(r"\alignedmarginpar{");
        text.push_str(&self.0.format_for_latex(config));
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

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        let mut text = String::from(r"\alignedmarginpar{");
        text.push_str(&self.0.format_for_latex(config));
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

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        if self.unit == "page" || self.unit == "speech" {
            return String::new();
        }

        if let Some(number) = &self.number {
            let mut text = String::from(r"\alignedmarginpar{");
            text.push_str(&number.format_for_latex(config));
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

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        let inner = self.text.format_for_latex(config);
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

    fn format_for_latex(&self, config: &FormatterConfig) -> String {
        format!(
            "{}\\footnote{{{}}} ",
            self.rend.as_ref().map(|x| x.as_str()).unwrap_or("[\\dots]"),
            ensure_dot(translate_gap_reason(&self.reason.format_for_latex(config)))
        )
    }
}

fn fix_punctuation(text: String, p: &str) -> String {
    // A quick way to normalize spaces. Much faster than regexes.
    text
        // To avoid using `format!`
        .replace(p, "\x00")
        // Block replacing before quotation marks
        .replace("\x00'", "\x01'")
        .replace("\x00\"", "\x01\"")
        // Normalize spaces everywhere
        .replace("\x00", "\x00 ") // Double spaces are fixed later in `fix_text`
        .replace(" \x00", "\x00")
        // Restore before quotation marks
        .replace("\x01", "\x00")
        // Normalize spaces next to quotation marks
        .replace(" \x00'", "\x00'")
        .replace(" \x00\"", "\x00\"")
        // Restore `p`
        .replace("\x00", p)
}

pub fn fix_text(mut text: String) -> String {
    text = fix_punctuation(text, ",");
    text = fix_punctuation(text, ".");
    text = fix_punctuation(text, "?");
    text = fix_punctuation(text, "!");
    text = fix_punctuation(text, ";");
    text = fix_punctuation(text, ";"); // Greek question mark
    text = fix_punctuation(text, ":");
    text = fix_punctuation(text, "·");

    text.replace("&gt;", "")
        .replace("&lt;", "") // Remove junk
        .replace(" — ", "---")
        .replace("— ", "---")
        .replace(" —", "---")
        .replace(" ---", "---")
        .replace("--- ", "---")
        // Fix multiple spaces
        .replace("  ", " ")
        .replace("   ", " ")
}

const WORD_ENDS: [&str; 7] = [" ", ".", ",", "!", "?", ";", ":"];

fn replace_word(text: String, word: &str, replacement: &str, terminator: &str) -> String {
    text.replace(
        &format!(" {word}{terminator}"),
        &format!(" {replacement}{terminator}"),
    )
}

fn replace_words(mut text: String, word: &str, replacement: &str) -> String {
    for terminator in WORD_ENDS {
        text = replace_word(text, word, replacement, terminator);
    }

    text
}

fn replace_et_ampersand(mut text: String) -> String {
    text = replace_words(text, "et", "\\&");
    text = replace_words(text, "etc", "\\&c");
    text
}

fn replace_ae_oe(mut text: String) -> String {
    text = text.replace("ae", "æ");
    text = text.replace("Ae", "Æ");
    text = text.replace("AE", "Æ");

    text = text.replace("oe", "œ");
    text = text.replace("Oe", "œ");
    text = text.replace("OE", "Œ");

    text
}

fn normalize_text(mut text: String) -> String {
    text = replace_et_ampersand(text);
    text = replace_ae_oe(text);
    text = text.replace('#', r"\#");
    text
}
