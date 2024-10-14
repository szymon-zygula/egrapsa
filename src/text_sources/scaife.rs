use super::{GetTextError, GetTextResult, TextSource};
use crate::text::{
    Footnote, Gap, LineNumber, Milestone, ParagraphNumber, TextNode, TextNodeKind, TextParent,
};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    name::QName,
    Reader,
};
use ureq;

pub struct Scaife {}

impl Scaife {
    fn text_url(id: &str) -> String {
        format!("https://scaife.perseus.org/library/{}/cts-api-xml", id)
    }
}

impl TextSource for Scaife {
    fn get_text(&self, id: &str) -> GetTextResult {
        let body = ureq::get(&Self::text_url(id))
            .call()
            .map_err(|_| GetTextError::ConnectionError)?
            .into_string()
            .map_err(|_| GetTextError::EncodingError)?;

        let mut out = std::fs::File::create("debug.xml").unwrap();
        std::io::Write::write_all(&mut out, body.as_bytes()).unwrap();

        let reader = &mut quick_xml::Reader::from_str(&body);
        reader.trim_text(true);
        let buf = &mut Vec::new();

        expect_opening_tag(reader, buf, "GetPassage");
        skip_expect_tag(reader, buf, "request");
        expect_opening_tag(reader, buf, "reply");
        skip_expect_tag(reader, buf, "urn");
        expect_opening_tag(reader, buf, "passage");
        expect_opening_tag(reader, buf, "TEI");
        expect_opening_tag(reader, buf, "text");
        expect_opening_tag(reader, buf, "body");

        let starting_div = read_starting_div(reader, buf).to_owned();
        let section = read_text(reader, buf, starting_div);

        expect_closing_tag(reader, buf, "body");
        expect_closing_tag(reader, buf, "text");
        expect_closing_tag(reader, buf, "TEI");
        expect_closing_tag(reader, buf, "passage");
        expect_closing_tag(reader, buf, "reply");
        expect_closing_tag(reader, buf, "GetPassage");

        expect_eof(reader, buf);

        // println!("{:#?}", section.remove_new_lines());

        Ok(section)
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

fn fix_text(text: &str) -> String {
    text.replace("&gt;", "")
        .replace("&lt;", "") // Remove junk
        .replace(",", ", ")
        .replace(",  ", ", ")
        .replace(",   ", ", ")
        .replace(".", ". ")
        .replace(".  ", ". ")
        .replace(".   ", ". ")
        .replace(" — ", "—")
        .replace("— ", "—")
        .replace(" —", "—")
        .replace("—", "---")
        .replace(" ?", "?")
        .replace("?  ", "? ")
        .replace(" ;", ";") // Greek question mark
        .replace(";  ", "; ")
        .replace(" :", ":") // Colon
        .replace(":  ", ": ")
        .replace(" ;", ";") // Semicolon
        .replace(";  ", "; ")
        .replace(" ·", "·") // Raised point (Greek colon)
        .replace("·  ", "· ")

    // A quick way to normalize spaces
}

fn read_text(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, start_tag: BytesStart) -> TextParent {
    let mut subtexts = Vec::<Box<dyn TextNode>>::new();
    let mut name: Option<Box<dyn TextNode>> = None;
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(tag)) => match name_to_str(&tag.name()) {
                "p" | "div" | "del" | "foreign" | "label" | "q" | "title" | "quote" | "l"
                | "cit" | "said" | "add" | "corr" | "num" | "sp" | "speaker" | "sic" | "reg"
                | "date" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(text));
                }
                "note" | "bibl" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(Footnote(text.to_string())));
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
            Ok(Event::Text(content)) => subtexts.push(Box::new(fix_text(
                std::str::from_utf8(&content.into_inner()).unwrap(),
            ))),
            Ok(Event::Empty(tag)) => subtexts.push(read_empty_tag(&tag)),
            Err(e) => panic!("Expected text, got error: {e}"),
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
        "l" => Box::new(""), // Sometimes <l/> appears for not reason. Seems to be some junk.
        "pb" => Box::new(ParagraphNumber(get_attr_val(&tag, "n"))),
        "lb" => Box::new(LineNumber(get_attr_val(&tag, "n"))),
        "gap" => {
            let reason = get_attr_val(tag, "reason");
            let rend = get_attr_val(tag, "rend");
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
            panic!("Unexpected tag found inside section: <{}>", name)
        }
    }
}

fn name_to_str<'a>(name: &QName<'a>) -> &'a str {
    std::str::from_utf8(name.0).unwrap()
}

fn get_text_kind(tag: &BytesStart) -> TextNodeKind {
    match name_to_str(&tag.name()) {
        "head" | "foreign" | "quote" | "add" => TextNodeKind::Simple,
        "date" => TextNodeKind::Date,
        "reg" => TextNodeKind::Regularized,
        "sp" => TextNodeKind::DialogueEntry,
        "sic" => TextNodeKind::Sic,
        "speaker" => TextNodeKind::Speaker,
        "num" => TextNodeKind::Symbol,
        "corr" => TextNodeKind::Corrected,
        "l" => TextNodeKind::Line,
        "label" => TextNodeKind::Label,
        "title" => TextNodeKind::Italics,
        "p" | "said" => TextNodeKind::Paragraph,
        "note" | "bibl" => TextNodeKind::Note,
        "del" => TextNodeKind::Deleted,
        "q" => TextNodeKind::Quote,
        "cit" => TextNodeKind::BlockQuote,
        "div" => match get_attr_val(tag, "type").as_str() {
            "edition" => TextNodeKind::Book,
            "textpart" => match get_attr_val(tag, "subtype").as_str() {
                // section -> paragraph is correct, it's basically how Scaife treats sections
                "section" => TextNodeKind::Paragraph,
                "book" => TextNodeKind::Section,
                "chapter" => TextNodeKind::Chapter,
                name => panic!("Invalid div subtype for text kind: {name}"),
            },
            name => panic!("Invalid div type for text kind: {name}"),
        },
        name => panic!("Invalid tag type for text kind: {name}"),
    }
}
