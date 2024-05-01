use super::{GetTextError, GetTextResult, TextSource};
use crate::text::TextTree;
use quick_xml::{events::Event, Reader};
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

        let mut reader = quick_xml::Reader::from_str(&body);
        let mut buf = Vec::new();

        parse_body(&mut reader, &mut buf)
    }
}

fn parse_body(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) -> GetTextResult {
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) if e.name().0 == "body".as_bytes() => break,
            Err(_) => panic!("Missing body"),
            _ => (),
        }
    }

    let body_span = reader
        .read_to_end(quick_xml::name::QName("body".as_bytes()))
        .unwrap();
    println!("element: {:?}", body_span);

    Ok(TextTree {
        name: None,
        subtexts: vec![],
    })
}
