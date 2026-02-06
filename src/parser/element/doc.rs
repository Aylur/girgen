use super::{AnyElement, Attrs, DocElement, ParseError};

pub struct DocFormat {
    // Valid values are: gi-docgen, gtk-doc-docbook, gtk-doc-markdown, hotdoc, unknown.
    pub name: String,
}

pub struct DocSection {
    pub name: String,
    pub elements: Vec<DocElement>,
}

impl super::Element for DocFormat {
    const KIND: &'static str = "doc-format";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
        })
    }
}

impl super::Element for DocSection {
    const KIND: &'static str = "doc-section";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
            elements: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        match DocElement::try_from_element(element) {
            Ok(ok) => {
                self.elements.push(ok);
                Ok(())
            }
            Err(ele) => Err(ParseError::UnexpectedElement(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}
