use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

pub struct Member {
    pub info: InfoAttrs,

    pub name: String,
    pub value: String,
    pub c_identifier: String,
    pub glib_nick: Option<String>,
    pub glib_name: Option<String>,

    pub info_elements: Vec<InfoElement>,
}

impl super::Element for Member {
    const KIND: &'static str = "member";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            value: attrs.get_string("value")?,
            c_identifier: attrs.get_string("c:identifier")?,
            glib_nick: attrs.get_string("glib:nick").ok(),
            glib_name: attrs.get_string("glib:name").ok(),
            info_elements: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        let element = match InfoElement::try_from_element(element) {
            Err(ele) => ele,
            Ok(ok) => {
                self.info_elements.push(ok);
                return Ok(());
            }
        };

        Err(ParseError::UnexpectedElement(format!(
            "{}:{}",
            Self::KIND,
            element.kind()
        )))
    }
}
