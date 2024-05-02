use super::{GetTextError, GetTextResult, TextSource};
use crate::text::{Footnote, LineNumber, ParagraphNumber, TextNode, TextTree};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
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

        println!("{}", body);

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
        expect_opening_div(reader, buf, "edition");

        try_read_section(reader, buf);

        expect_closing_tag(reader, buf, "body");
        expect_closing_tag(reader, buf, "text");
        expect_closing_tag(reader, buf, "TEI");
        expect_closing_tag(reader, buf, "passage");
        expect_closing_tag(reader, buf, "reply");
        expect_closing_tag(reader, buf, "GetPassage");

        expect_eof(reader, buf);

        Ok(TextTree {
            name: None,
            subtexts: vec![],
        })
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

fn expect_opening_div(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, div_type: &str) {
    let div = expect_opening_tag(reader, buf, "div");
    expect_attribute(&div, "type", div_type);
}

fn expect_attribute(tag: &BytesStart, attr_name: &str, attr_value: &str) {
    if *tag.try_get_attribute(attr_name).unwrap().unwrap().value != *attr_value.as_bytes() {
        panic!("Expected <... {attr_name}=\"{attr_value}\" ...>, got {tag:?}");
    }
}

fn try_read_section(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) {
    let tag = expect_opening_tag(reader, buf, "div");
    expect_attribute(&tag, "type", "textpart");
    expect_attribute(&tag, "subtype", "section");

    let mut paragraphs = Vec::<Box<dyn TextNode>>::new();
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(tag)) => match std::str::from_utf8(tag.name().0).unwrap() {
                "p" => {
                    let text = try_read_text(reader, buf, "p");
                    paragraphs.push(Box::new(text));
                }
                "note" => {
                    expect_attribute(&tag, "type", "footnote");
                    let text = try_read_text(reader, buf, "note");
                    paragraphs.push(Box::new(Footnote(text)));
                }
                name @ _ => {
                    panic!("Unexpected tag found inside section: {:?}", name)
                }
            },
            Ok(Event::End(tag)) => {
                ensure_end_tag_name(&tag, "div");
                break;
            }
            Ok(Event::Empty(tag)) => match std::str::from_utf8(tag.name().0).unwrap() {
                "pb" => {
                    let n: u32 = get_attr_val(&tag, "n").parse().unwrap();
                    paragraphs.push(Box::new(ParagraphNumber(n)));
                }
                "lb" => {
                    let n: u32 = get_attr_val(&tag, "n").parse().unwrap();
                    paragraphs.push(Box::new(LineNumber(n)));
                }
                name @ _ => {
                    panic!("Unexpected tag found inside section: {:?}", name)
                }
            },
            Err(e) => panic!("Expected text, got error: {e}"),
            ev => panic!("Missing text, got event: {ev:?}"),
        }
    }

    println!("{:?}", paragraphs);
    expect_closing_tag(reader, buf, "div");
}

// Expects starting_tag to be read already
fn try_read_text(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, tag: &str) -> String {
    let mut text = String::new();
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Text(content)) => text += std::str::from_utf8(&content.into_inner()).unwrap(),
            Ok(Event::End(end)) => {
                ensure_end_tag_name(&end, tag);
                break;
            }
            // TODO: Don't get rid of line numbers?
            Ok(Event::Empty(tag)) => ensure_empty_tag_innocuous(tag),
            Err(e) => panic!("Expected text, got error: {e}"),
            ev => panic!("Missing text, got event: {ev:?}"),
        }
    }

    text
}

fn ensure_tag_name(tag: &BytesStart, name: &str) {
    if tag.name().0 != name.as_bytes() {
        panic!("Expected opening tag <{name}>, found {:?}", tag.name());
    }
}

fn ensure_end_tag_name(tag: &BytesEnd, name: &str) {
    if tag.name().0 != name.as_bytes() {
        panic!("Expected closing tag </{name}>, found {:?}", tag.name());
    }
}

fn ensure_empty_tag_innocuous(tag: BytesStart) {
    if tag.name().0 == "lb".as_bytes() {
        return;
    }

    panic!("Unexpected tag found in text: {:?}", tag.name())
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
