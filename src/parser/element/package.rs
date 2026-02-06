use super::{Attrs, ParseError};

pub struct Package {
    pub name: String,
}

impl super::Element for Package {
    const KIND: &'static str = "package";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Package {
            name: attrs.get_string("name")?,
        })
    }
}
