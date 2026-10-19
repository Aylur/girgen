use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidGirElement(String),
    MalformedGir(&'static str),
    UnexpectedElement(&'static str, &'static str),
    MissingAttribute(String),
    XmlError(quick_xml::Error),
    AttrError(quick_xml::events::attributes::AttrError),
}

impl From<quick_xml::Error> for ParseError {
    fn from(err: quick_xml::Error) -> Self {
        ParseError::XmlError(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidGirElement(tag) => write!(f, "unknown GIR element: {tag}"),
            ParseError::MalformedGir(msg) => write!(f, "malformed GIR: {msg}"),
            ParseError::UnexpectedElement(parent, child) => {
                write!(f, "unexpected element: {parent}>{child}")
            }
            ParseError::MissingAttribute(attr) => write!(f, "missing attribute: {attr}"),
            ParseError::XmlError(err) => write!(f, "XML error: {err}"),
            ParseError::AttrError(err) => write!(f, "decoding error: {err}"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::XmlError(e) => Some(e),
            _ => None,
        }
    }
}
