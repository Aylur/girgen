use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

pub struct Signal {
    pub info: InfoAttrs,
    pub name: String,
    pub detailed: Option<bool>,
    pub when: Option<String>,
    pub action: Option<bool>,
    pub no_hooks: Option<bool>,
    pub no_recurse: Option<bool>,
    pub emitter: Option<String>,

    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
}

impl super::Element for Signal {
    const KIND: &'static str = "signal";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            detailed: attrs.get_boolean("detailed").ok(),
            when: attrs.get_string("when").ok(),
            action: attrs.get_boolean("action").ok(),
            no_hooks: attrs.get_boolean("no_hooks").ok(),
            no_recurse: attrs.get_boolean("no_recurse").ok(),
            emitter: attrs.get_string("emitter").ok(),
            info_elements: Vec::new(),
            parameters: None,
            return_value: None,
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
