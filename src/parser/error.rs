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
