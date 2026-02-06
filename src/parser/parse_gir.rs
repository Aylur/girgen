use super::element::{AnyElement, Attrs, Element, Repository};
use super::error::ParseError;
use quick_xml::events::{BytesStart, Event};

fn attributes(e: &BytesStart) -> Result<Attrs, ParseError> {
    let mut attrs = std::collections::HashMap::new();

    for attr in e.attributes() {
        match attr {
            Err(_) => todo!(),
            Ok(a) => {
                let key = String::from_utf8(a.key.into_inner().to_vec())?;
                let value = a.unescape_value()?.into_owned();
                attrs.insert(key, value);
            }
        }
    }

    Ok(Attrs(attrs))
}

impl AnyElement {
    fn push(&mut self, e: &BytesStart) -> Result<Self, ParseError> {
        let attrs = attributes(e)?;
        AnyElement::new(e.name().as_ref(), &attrs)
    }
}

pub fn parse(gir_contents: &str) -> Result<Repository, ParseError> {
    let mut reader = quick_xml::Reader::from_str(gir_contents);

    let mut repo: Option<Repository> = None;
    let mut stack: Vec<AnyElement> = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"repository" => {
                    let attrs = attributes(&e)?;
                    stack.push(AnyElement::Repository(Repository::new(&attrs)?));
                }
                _ => {
                    let ele = stack
                        .last_mut()
                        .ok_or(ParseError::MalformedGir("failed to push: stack is empty"))?
                        .push(&e)?;

                    stack.push(ele);
                }
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"repository" => {
                    if let AnyElement::Repository(r) = stack.pop().ok_or(
                        ParseError::MalformedGir("root element expected to be repository"),
                    )? {
                        repo = Some(r);
                    }
                }
                _ => {
                    let top = stack
                        .pop()
                        .ok_or(ParseError::MalformedGir("failed to end: stack pop"))?;

                    let second = stack
                        .last_mut()
                        .ok_or(ParseError::MalformedGir("failed to end: stack is empty"))?;

                    second.end(top)?;
                }
            },
            Ok(Event::Empty(e)) => {
                let top = stack
                    .last_mut()
                    .ok_or(ParseError::MalformedGir("failed empty()"))?;

                let new = top.push(&e)?;
                top.end(new)?;
            }
            Ok(Event::Text(e)) => {
                if let Some(top) = stack.last_mut() {
                    let content = e.xml_content()?;
                    top.text(content.as_ref())?;
                }
            }
            Ok(Event::GeneralRef(e)) => {
                let top = stack
                    .last_mut()
                    .ok_or(ParseError::MalformedGir("general ref text"))?;

                let ch = match e.xml_content()?.as_ref() {
                    "lt" => "<",
                    "gt" => ">",
                    "amp" => "'",
                    c => todo!("write ref event {}", c),
                };

                top.text(ch)?;
            }
            Ok(Event::CData(_)) => {
                todo!()
            }
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }
    }

    repo.ok_or(ParseError::MalformedGir("missing repo"))
}
