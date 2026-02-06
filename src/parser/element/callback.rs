use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

pub struct Callback {
    pub info: InfoAttrs,
    pub name: String,
    pub c_type: Option<String>,
    pub throws: Option<bool>,

    pub info_elements: Vec<InfoElement>,
    pub params: Option<super::Parameters>,
    pub r#return: Option<super::ReturnValue>,
}

impl super::Element for Callback {
    const KIND: &'static str = "callback";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            c_type: attrs.get_string("c:type").ok(),
            throws: attrs.get_boolean("throws").ok(),
            info_elements: Vec::new(),
            params: None,
            r#return: None,
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

        match element {
            AnyElement::Parameters(p) => {
                self.params = Some(p);
                Ok(())
            }
            AnyElement::ReturnValue(r) => {
                self.r#return = Some(r);
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
