use super::{AnyElement, AnyType, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Alias {
    pub info: InfoAttrs,
    pub name: String,
    pub c_type: String,

    pub info_elements: Vec<InfoElement>,
    pub r#type: Option<AnyType>,
}

impl super::Element for Alias {
    const KIND: &'static str = "alias";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            c_type: attrs.get_string("c:type")?,
            info_elements: Vec::new(),
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

        Err(ParseError::UnexpectedElement(Self::KIND, element.kind()))
    }
}
