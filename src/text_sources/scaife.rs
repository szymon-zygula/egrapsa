use super::{GetTextError, GetTextResult, TextSource};
use crate::text::{Footnote, Gap, LineNumber, ParagraphNumber, TextNode, TextNodeKind, TextParent};
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

        // println!("{}", body);

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

fn read_text(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, start_tag: BytesStart) -> TextParent {
    let mut subtexts = Vec::<Box<dyn TextNode>>::new();
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(tag)) => match name_to_str(&tag.name()) {
                "p" | "div" | "del" | "foreign" | "label" | "q" | "title" | "quote" | "l"
                | "cit" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(text));
                }
                "note" | "bibl" => {
                    let tag = tag.to_owned();
                    let text = read_text(reader, buf, tag);
                    subtexts.push(Box::new(Footnote(text.to_string())));
                }
                name @ _ => {
                    panic!("Unexpected tag found inside section: {:?}", name)
                }
            },
            Ok(Event::End(tag)) => {
                ensure_tag_end(&tag, &start_tag);
                break;
            }
            Ok(Event::Text(content)) => subtexts.push(Box::new(
                std::str::from_utf8(&content.into_inner())
                    .unwrap()
                    .to_string(),
            )),
            Ok(Event::Empty(tag)) => subtexts.push(read_emty_tag(&tag)),
            Err(e) => panic!("Expected text, got error: {e}"),
            ev => panic!("Missing text, got event: {ev:?}"),
        }
    }

    TextParent {
        name: None,
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

fn expect_eof(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) {
    let event = reader.read_event_into(buf).unwrap();

    if event != Event::Eof {
        panic!("Expected EOF, found: {event:?}")
    }
}

fn read_emty_tag(tag: &BytesStart) -> Box<dyn TextNode> {
    match name_to_str(&tag.name()) {
        "pb" => {
            let n: u32 = to_u32(&get_attr_val(&tag, "n"));
            Box::new(ParagraphNumber(n))
        }
        "lb" => {
            let n: u32 = to_u32(&get_attr_val(&tag, "n"));
            Box::new(LineNumber(n))
        }
        "gap" => {
            let reason = get_attr_val(tag, "reason");
            Box::new(Gap(reason))
        }
        "milestone" => Box::new(String::new()),
        "space" => Box::new(" "),
        name @ _ => {
            panic!("Unexpected tag found inside section: {:?}", name)
        }
    }
}

fn name_to_str<'a>(name: &QName<'a>) -> &'a str {
    std::str::from_utf8(name.0).unwrap()
}

// Sometimes paragraph/line numbers include stray characters, e.g. "15."
fn to_u32(string: &str) -> u32 {
    string
        .trim()
        .trim_matches(|c: char| !c.is_digit(10))
        .parse()
        .unwrap()
}

fn get_text_kind(tag: &BytesStart) -> TextNodeKind {
    match name_to_str(&tag.name()) {
        "foreign" => TextNodeKind::Simple,
        "label" => TextNodeKind::Label,
        "title" => TextNodeKind::Italics,
        "p" => TextNodeKind::Paragraph,
        "note" | "bibl" => TextNodeKind::Note,
        "del" => TextNodeKind::Deleted,
        "l" => TextNodeKind::Simple,
        "quote" => TextNodeKind::Simple,
        "q" => TextNodeKind::Quote,
        "cit" => TextNodeKind::BlockQuote,
        "div" => match get_attr_val(tag, "type").as_str() {
            "edition" => TextNodeKind::Book,
            "textpart" => match get_attr_val(tag, "subtype").as_str() {
                // section -> paragraph is correct, it's basically how Scaife treats sections
                "section" => TextNodeKind::Paragraph,
                "chapter" => TextNodeKind::Chapter,
                name => panic!("Invalid div subtype for text kind: {name}"),
            },
            name => panic!("Invalid div type for text kind: {name}"),
        },
        name => panic!("Invalid tag type for text kind: {name}"),
    }
}
