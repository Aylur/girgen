use super::{AnyElement, AnyType, Attrs, InfoAttrs, InfoElement, ParseError};

pub struct Field {
    pub info: InfoAttrs,
    pub name: String,
    pub writable: Option<bool>,
    pub readable: Option<bool>,
    pub private: Option<bool>,
    pub bits: Option<i32>,

    pub info_elements: Vec<InfoElement>,
    pub callback: Option<super::Callback>,
    pub r#type: Option<AnyType>,
}

impl super::Element for Field {
    const KIND: &'static str = "field";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            writable: attrs.get_boolean("writable").ok(),
            readable: attrs.get_boolean("readable").ok(),
            private: attrs.get_boolean("private").ok(),
            bits: attrs.get_int("bits").ok(),
            info_elements: Vec::new(),
            callback: None,
            r#type: None,
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

        let element = match AnyType::try_from_element(element) {
            Err(ele) => ele,
            Ok(ok) => {
                self.r#type = Some(ok);
                return Ok(());
            }
        };

        match element {
            AnyElement::Callback(c) => {
                self.callback = Some(c);
                Ok(())
            }
            ele => Err(ParseError::UnexpectedElement(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}
