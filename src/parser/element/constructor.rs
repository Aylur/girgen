use super::{AnyElement, Attrs, CallableAttrs, InfoElement, ParseError};

pub struct Constructor {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub params: Option<super::Parameters>,
    pub r#return: Option<super::ReturnValue>,
}

impl super::Element for Constructor {
    const KIND: &'static str = "constructor";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
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
