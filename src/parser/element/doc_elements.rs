use super::{AnyElement, Attrs, ParseError};

pub struct DocVersion {
    pub xml_space: Option<String>,
    pub xml_whitespace: Option<String>,
    pub text: String,
}

pub struct DocStability {
    pub xml_space: Option<String>,
    pub xml_whitespace: Option<String>,
    pub text: String,
}

pub struct Doc {
    pub xml_space: Option<String>,
    pub xml_whitespace: Option<String>,
    pub filename: String,
    pub line: String,
    pub column: Option<String>,
    pub text: String,
}

pub struct DocDeprecated {
    pub xml_space: Option<String>,
    pub xml_whitespace: Option<String>,
    pub text: String,
}

pub struct SourcePosition {
    pub filename: String,
    pub line: String,
    pub column: Option<String>,
}

pub enum DocElement {
    DocVersion(DocVersion),
    DocStability(DocStability),
    Doc(Doc),
    DocDeprecated(DocDeprecated),
    SourcePosition(SourcePosition),
}

impl DocElement {
    #[allow(clippy::result_large_err)]
    pub fn try_from_element(element: AnyElement) -> Result<Self, AnyElement> {
        match element {
            AnyElement::DocVersion(e) => Ok(Self::DocVersion(e)),
            AnyElement::DocStability(e) => Ok(Self::DocStability(e)),
            AnyElement::Doc(e) => Ok(Self::Doc(e)),
            AnyElement::DocDeprecated(e) => Ok(Self::DocDeprecated(e)),
            AnyElement::SourcePosition(e) => Ok(Self::SourcePosition(e)),
            e => Err(e),
        }
    }
}

impl super::Element for DocVersion {
    const KIND: &'static str = "doc-version";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            xml_space: attrs.get_string("xml:space").ok(),
            xml_whitespace: attrs.get_string("xml:whitespace").ok(),
            text: String::from(""),
        })
    }

    fn text(&mut self, str: &str) -> Result<(), ParseError> {
        self.text = String::from(str);
        Ok(())
    }
}

impl super::Element for DocStability {
    const KIND: &'static str = "doc-stability";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            xml_space: attrs.get_string("xml:space").ok(),
            xml_whitespace: attrs.get_string("xml:whitespace").ok(),
            text: String::from(""),
        })
    }

    fn text(&mut self, str: &str) -> Result<(), ParseError> {
        self.text = String::from(str);
        Ok(())
    }
}

impl super::Element for Doc {
    const KIND: &'static str = "doc";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            xml_space: attrs.get_string("xml:space").ok(),
            xml_whitespace: attrs.get_string("xml:whitespace").ok(),
            filename: attrs.get_string("filename")?,
            line: attrs.get_string("line")?,
            column: attrs.get_string("line").ok(),
            text: String::from(""),
        })
    }

    fn text(&mut self, str: &str) -> Result<(), ParseError> {
        self.text = String::from(str);
        Ok(())
    }
}

impl super::Element for DocDeprecated {
    const KIND: &'static str = "doc-deprecated";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            xml_space: attrs.get_string("xml:space").ok(),
            xml_whitespace: attrs.get_string("xml:whitespace").ok(),
            text: String::from(""),
        })
    }

    fn text(&mut self, str: &str) -> Result<(), ParseError> {
        self.text = String::from(str);
        Ok(())
    }
}

impl super::Element for SourcePosition {
    const KIND: &'static str = "source-position";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            filename: attrs.get_string("filename")?,
            line: attrs.get_string("line")?,
            column: attrs.get_string("column").ok(),
        })
    }
}
