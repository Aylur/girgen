use super::{AnyElement, Attrs, CallableAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Constructor {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
}

impl super::Element for Constructor {
    const KIND: &'static str = "constructor";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            info_elements: Vec::new(),
            parameters: None,
            return_value: None,
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
                self.parameters = Some(p);
                Ok(())
            }
            AnyElement::ReturnValue(r) => {
                self.return_value = Some(r);
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
