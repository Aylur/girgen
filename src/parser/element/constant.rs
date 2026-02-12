use super::{AnyElement, AnyType, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Constant {
    pub info: InfoAttrs,
    pub name: String,
    pub value: String,
    pub c_type: Option<String>,
    pub c_identifier: Option<String>,
    pub r#type: Option<AnyType>,
    pub info_elements: Vec<InfoElement>,
}

impl super::Element for Constant {
    const KIND: &'static str = "constant";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            value: attrs.get_string("value")?,
            c_type: attrs.get_string("c:type").ok(),
            c_identifier: attrs.get_string("c:identifier").ok(),
            r#type: None,
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
