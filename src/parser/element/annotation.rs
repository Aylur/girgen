use super::{Attrs, ParseError};

pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl super::Element for Attribute {
    const KIND: &'static str = "attribute";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
            value: attrs.get_string("value")?,
        })
    }
}
