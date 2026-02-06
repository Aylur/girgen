use super::{Attrs, ParseError};

pub struct CInclude {
    pub name: String,
}

impl super::Element for CInclude {
    const KIND: &'static str = "c:include";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
        })
    }
}
