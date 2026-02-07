use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnhandledXmlTag(String),
    MalformedGir(&'static str),
    UnexpectedElement(String),
    MissingAttribute(String),
    XmlError(quick_xml::Error),
    EncodeError(quick_xml::encoding::EncodingError),
    Utf8Error(std::string::FromUtf8Error),
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ParseError::Utf8Error(err)
    }
}

impl From<quick_xml::Error> for ParseError {
    fn from(err: quick_xml::Error) -> Self {
        ParseError::XmlError(err)
    }
}

impl From<quick_xml::encoding::EncodingError> for ParseError {
    fn from(err: quick_xml::encoding::EncodingError) -> Self {
        ParseError::EncodeError(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnhandledXmlTag(tag) => write!(f, "unhandled XML tag: {tag}"),
            ParseError::MalformedGir(msg) => write!(f, "malformed GIR: {msg}"),
            ParseError::UnexpectedElement(el) => write!(f, "unexpected element: {el}"),
            ParseError::MissingAttribute(attr) => write!(f, "missing attribute: {attr}"),
            ParseError::XmlError(err) => write!(f, "XML error: {err}"),
            ParseError::EncodeError(err) => write!(f, "encoding error: {err}"),
            ParseError::Utf8Error(err) => write!(f, "utf-8 error: {err}"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::XmlError(e) => Some(e),
            ParseError::EncodeError(e) => Some(e),
            ParseError::Utf8Error(e) => Some(e),
            _ => None,
        }
    }
}
