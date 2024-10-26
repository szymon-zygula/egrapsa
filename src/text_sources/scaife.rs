use super::{GetTextError, GetTextResult, TextSource};
use crate::text::{
    fix_text, Footnote, Gap, Highlight, LineNumber, MarginNote, Milestone, ParagraphNumber,
    TextNode, TextNodeKind, TextParent,
};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    name::QName,
    Reader,
};
use ureq;

trait ScaifeSource {
    fn open(&self, reader: &mut quick_xml::Reader<&[u8]>, buf: &mut Vec<u8>);
    fn close(&self, reader: &mut quick_xml::Reader<&[u8]>, buf: &mut Vec<u8>);
    fn text(&self) -> &str;
}

struct ScaifeFile {
    text: String,
}

impl ScaifeSource for ScaifeFile {
    fn open(&self, reader: &mut quick_xml::Reader<&[u8]>, buf: &mut Vec<u8>) {
        skip_expect_decl(reader, buf);
        skip_expect_pi(reader, buf);
        expect_opening_tag(reader, buf, "TEI");
        skip_expect_tag(reader, buf, "teiHeader");
        expect_opening_tag(reader, buf, "text");
        expect_opening_tag(reader, buf, "body");
    }

    fn close(&self, reader: &mut quick_xml::Reader<&[u8]>, buf: &mut Vec<u8>) {
        expect_closing_tag(reader, buf, "body");
        expect_closing_tag(reader, buf, "text");
        expect_closing_tag(reader, buf, "TEI");
        expect_eof(reader, buf);
    }

    fn text(&self) -> &str {
        &self.text
    }
}

struct ScaifeUrn {
    text: String,
}

impl ScaifeSource for ScaifeUrn {
    fn open(&self, reader: &mut quick_xml::Reader<&[u8]>, buf: &mut Vec<u8>) {
        expect_opening_tag(reader, buf, "GetPassage");
        skip_expect_tag(reader, buf, "request");
        expect_opening_tag(reader, buf, "reply");
        skip_expect_tag(reader, buf, "urn");
        expect_opening_tag(reader, buf, "passage");
        expect_opening_tag(reader, buf, "TEI");
        expect_opening_tag(reader, buf, "text");
        expect_opening_tag(reader, buf, "body");
    }

    fn close(&self, reader: &mut quick_xml::Reader<&[u8]>, buf: &mut Vec<u8>) {
        expect_closing_tag(reader, buf, "body");
        expect_closing_tag(reader, buf, "text");
        expect_closing_tag(reader, buf, "TEI");
        expect_closing_tag(reader, buf, "passage");
        expect_closing_tag(reader, buf, "reply");
        expect_closing_tag(reader, buf, "GetPassage");
        expect_eof(reader, buf);
    }

    fn text(&self) -> &str {
        &self.text
    }
}

pub struct Scaife {}

impl Scaife {
    fn text_url(id: &str) -> String {
        format!("https://scaife.perseus.org/library/{}/cts-api-xml", id)
    }

    fn id_to_source(&self, id: &str) -> Result<Box<dyn ScaifeSource>, GetTextError> {
        Ok(if id.starts_with("urn") {
            Box::new(ScaifeUrn {
                text: ureq::get(&Self::text_url(id))
                    .call()
                    .map_err(|_| GetTextError::ConnectionError)?
                    .into_string()
                    .map_err(|_| GetTextError::EncodingError)?,
            })
        } else if let Some(path) = id.strip_prefix("file:") {
            println!("Path: {path}");
            Box::new(ScaifeFile {
                text: std::fs::read_to_string(std::path::Path::new(path))
                    .map_err(|_| GetTextError::FileSystemError)?,
            })
        } else {
            panic!("Invalid Scaife identifier prefix")
        })
    }
}

impl TextSource for Scaife {
    fn get_text(&self, id: &str) -> GetTextResult {
        let source = self.id_to_source(id)?;

        let mut out = std::fs::File::create("debug.xml").unwrap();
        std::io::Write::write_all(&mut out, source.text().as_bytes()).unwrap();

        let reader = &mut quick_xml::Reader::from_str(source.text());
        reader.trim_text(true);
        let buf = &mut Vec::new();

        source.open(reader, buf);

        let starting_div = read_starting_div(reader, buf).to_owned();
        reader.trim_text(false);
        let text = read_text(reader, buf, starting_div);
        reader.trim_text(true);

        source.close(reader, buf);

        Ok(text)
    }
}

fn expect_opening_tag<'a>(
    reader: &mut Reader<&[u8]>,
    buf: &'a mut Vec<u8>,
    tag_name: &str,
) -> BytesStart<'a> {
    match reader.read_event_into(buf) {
        Ok(Event::Start(e)) if e.name().0 == tag_name.as_bytes() => e,
        Err(e) => panic!("Expected tag <{tag_name}>, got error: {e}"),
        ev => panic!("Missing tag <{tag_name}>, got event: {ev:?}"),
    }
}

fn skip_expect_tag(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, tag_name: &str) {
    let bytes_start = expect_opening_tag(reader, buf, tag_name);

    reader
        .read_to_end(bytes_start.name())
        .map_err(|e| panic!("Could not read the whole <{tag_name}> tag, got error: {e}"))
        .unwrap();
}

fn skip_expect_decl(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) {
    let ev = reader.read_event_into(buf);
    if !matches!(ev, Ok(Event::Decl(_))) {
        panic!("Expected XML declaration, found {ev:?}")
    };
}

fn skip_expect_pi(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) {
    let ev = reader.read_event_into(buf);
    if !matches!(ev, Ok(Event::PI(_))) {
        panic!("Expected XML processing instruction, found {ev:?}")
    };
}

fn expect_closing_tag<'a>(reader: &mut Reader<&[u8]>, buf: &'a mut Vec<u8>, tag_name: &str) {
    match reader.read_event_into(buf) {
        Ok(Event::End(e)) if e.name().0 == tag_name.as_bytes() => (),
        Err(e) => panic!("Expected tag </{tag_name}>, got error: {e}"),
        ev => panic!("Missing tag </{tag_name}>, got event: {ev:?}"),
    }
}

fn read_starting_div<'a>(reader: &mut Reader<&[u8]>, buf: &'a mut Vec<u8>) -> BytesStart<'a> {
    match reader.read_event_into(buf) {
        Ok(Event::Start(tag)) => tag,
        other => panic!("Expected opening <div> tag, found {:?}", other),
    }
}

fn remove_unnecessary_whitespace(text: String) -> String {
    // New lines there don't mean anything for the text, same with tabulation
    text.replace('\n', " ").replace('\t', "")
}

fn read_text(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, start_tag: BytesStart) -> TextParent {
    let mut subtexts = Vec::<Box<dyn TextNode>>::new();
    let mut name: Option<Box<dyn TextNode>> = None;
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(tag)) => match name_to_str(&tag.name()).to_lowercase().as_str() {
                "p" | "div" | "del" | "foreign" | "label" | "q" | "title" | "quote" | "l"
                | "cit" | "said" | "add" | "corr" | "num" | "sp" | "speaker" | "sic" | "reg"
                | "ref" | "date" | "app" | "lem" | "choice" | "abbr" | "ex" | "expan" | "desc"
                | "persname" | "name" | "placename" | "rs" | "term" | "emph" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(text));
                }
                "note" | "bibl" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(Footnote(text.to_string())));
                }
                "gap" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new("[...]"));
                    subtexts.push(Box::new(Footnote(text.to_string())));
                }
                "hi" => {
                    let rend = get_attr_val(&tag, "rend");
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(Highlight {
                        rend,
                        text: Box::new(text),
                    }));
                }
                "head" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    name = Some(Box::new(text));
                }
                name @ _ => {
                    panic!("Unexpected tag found inside section: <{}>", name)
                }
            },
            Ok(Event::End(tag)) => {
                ensure_tag_end(&tag, &start_tag);
                break;
            }
            Ok(Event::Text(content)) => {
                subtexts.push(Box::new(fix_text(remove_unnecessary_whitespace(
                    std::str::from_utf8(&content.into_inner())
                        .unwrap()
                        .to_string(),
                ))))
            }
            Ok(Event::Empty(tag)) => subtexts.push(read_empty_tag(&tag)),
            Err(e) => panic!("Expected text, got error: {e}"),
            Ok(Event::Comment(_)) => {}
            ev => panic!("Missing text, got event: {ev:?}"),
        }
    }

    TextParent {
        name,
        kind: get_text_kind(&start_tag),
        subtexts,
    }
}

fn ensure_tag_end(tag: &BytesEnd, start_tag: &BytesStart) {
    if tag.name() != start_tag.name() {
        panic!(
            "Expected closing tag {:?}, found {:?}",
            start_tag.name(),
            tag.name()
        );
    }
}

fn get_attr_val(tag: &BytesStart, name: &str) -> String {
    std::str::from_utf8(&tag.try_get_attribute(name).unwrap().unwrap().value)
        .unwrap()
        .to_string()
}

fn get_attr_val_opt(tag: &BytesStart, name: &str) -> Option<String> {
    tag.try_get_attribute(name)
        .unwrap()
        .map(|attr| std::str::from_utf8(&attr.value).unwrap().to_string())
}

fn expect_eof(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) {
    let event = reader.read_event_into(buf).unwrap();

    if event != Event::Eof {
        panic!("Expected EOF, found: {event:?}")
    }
}

fn read_empty_tag(tag: &BytesStart) -> Box<dyn TextNode> {
    match name_to_str(&tag.name()) {
        // Sometimes <X /> appears for not reason,
        // where X should never be an empty tag.
        // Seems to be some junk.
        "l" | "p" => Box::new(""),
        "pb" => {
            if let Some(x) = get_attr_val_opt(&tag, "n") {
                Box::new(ParagraphNumber(x))
            } else {
                Box::new("")
            }
        }
        "lb" => {
            if let Some(x) = get_attr_val_opt(&tag, "n") {
                Box::new(LineNumber(x))
            } else {
                Box::new("")
            }
        }
        "note" => Box::new(MarginNote(get_attr_val(&tag, "n"))),
        "gap" => {
            let reason = get_attr_val(tag, "reason");
            let rend = get_attr_val_opt(tag, "rend");
            Box::new(Gap { reason, rend })
        }
        "milestone" => {
            let unit = get_attr_val(tag, "unit");
            let number = get_attr_val_opt(tag, "n");
            let ed = get_attr_val_opt(tag, "ed");
            let resp = get_attr_val_opt(tag, "resp");
            Box::new(Milestone {
                unit,
                number,
                ed,
                resp,
            })
        }
        "space" => Box::new(" "),
        name @ _ => {
            panic!("Unexpected empty tag found inside section: <{}/>", name)
        }
    }
}

fn name_to_str<'a>(name: &QName<'a>) -> &'a str {
    std::str::from_utf8(name.0).unwrap()
}

fn get_text_kind(tag: &BytesStart) -> TextNodeKind {
    match name_to_str(&tag.name()).to_lowercase().as_str() {
        "head" | "foreign" | "quote" | "add" => TextNodeKind::Simple,
        "date" => TextNodeKind::Date,
        "app" => TextNodeKind::Apparatus,
        "lem" => TextNodeKind::Lemma,
        "reg" => TextNodeKind::Regularized,
        "ref" => TextNodeKind::Ref,
        "choice" => TextNodeKind::Choice,
        "abbr" => TextNodeKind::Abbreviated,
        "ex" => TextNodeKind::Expanded,
        "expan" => TextNodeKind::Expandable,
        "sp" => TextNodeKind::DialogueEntry,
        "sic" => TextNodeKind::Sic,
        "speaker" => TextNodeKind::Speaker,
        "num" => TextNodeKind::Symbol,
        "corr" => TextNodeKind::Corrected,
        "name" => TextNodeKind::Name,
        "rs" => TextNodeKind::ReferencingString,
        "desc" => TextNodeKind::Description,
        "l" => TextNodeKind::Line,
        "label" => TextNodeKind::Label,
        "title" => TextNodeKind::Italics,
        "persname" => TextNodeKind::PersonName,
        "placename" => TextNodeKind::PlaceName,
        "term" => TextNodeKind::TechnicalTerm,
        "emph" => TextNodeKind::Emphasis,
        "hi" => TextNodeKind::Highlight,
        "p" | "said" => TextNodeKind::Paragraph,
        "gap" | "note" | "bibl" => TextNodeKind::Note,
        "del" => TextNodeKind::Deleted,
        "q" => TextNodeKind::Quote,
        "cit" => TextNodeKind::BlockQuote,
        "div" => match get_attr_val(tag, "type").to_lowercase().as_str() {
            "edition" => TextNodeKind::Book,
            "textpart" => match get_attr_val(tag, "subtype").to_lowercase().as_str() {
                // section -> paragraph is correct, it's basically how Scaife treats sections
                "epigram" => TextNodeKind::Epigram,
                // No idea why "textpart" appears as "subtype" sometimes
                "textpart" | "section" => TextNodeKind::Paragraph,
                "book" => TextNodeKind::Section,
                "chapter" => TextNodeKind::Chapter,
                "actio" => TextNodeKind::Chapter,
                name => panic!("Invalid div subtype for text kind: {name}"),
            },
            name => panic!("Invalid div type for text kind: {name}"),
        },
        name => panic!("Invalid tag type for text kind: {name}"),
    }
}
