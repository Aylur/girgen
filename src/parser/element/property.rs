use super::{AnyType, Attrs, InfoAttrs, InfoElement, ParseError};

pub struct Property {
    pub info: InfoAttrs,
    pub name: String,
    pub writable: Option<bool>,
    pub readable: Option<bool>,
    pub construct: Option<bool>,
    pub construct_only: Option<bool>,
    pub setter: Option<String>,
    pub getter: Option<String>,
    pub default_value: Option<String>,

    pub r#type: Option<AnyType>,
    pub info_elements: Vec<InfoElement>,
}

impl super::Element for Property {
    const KIND: &'static str = "property";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            writable: attrs.get_boolean("writable").ok(),
            readable: attrs.get_boolean("readable").ok(),
            construct: attrs.get_boolean("construct").ok(),
            construct_only: attrs.get_boolean("construct_only").ok(),
            setter: attrs.get_string("setter").ok(),
            getter: attrs.get_string("getter").ok(),
            default_value: attrs.get_string("default-value").ok(),
            r#type: None,
            info_elements: Vec::new(),
        })
    }

    fn end(&mut self, element: super::AnyElement) -> Result<(), ParseError> {
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

        Err(ParseError::UnhandledXmlTag(format!(
            "{}:{}",
            Self::KIND,
            element.kind()
        )))
    }
}
