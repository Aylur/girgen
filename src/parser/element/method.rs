use super::{AnyElement, Attrs, CallableAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Method {
    pub attrs: CallableAttrs,
    pub glib_set_property: Option<String>,
    pub glib_get_property: Option<String>,
    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
}

#[derive(Debug, Clone)]
pub struct MethodInline {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
}

#[derive(Debug, Clone)]
pub struct VirtualMethod {
    pub attrs: CallableAttrs,
    pub invoker: Option<String>,
    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
}

impl super::Element for Method {
    const KIND: &'static str = "method";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            glib_set_property: attrs.get_string("glib:set-property").ok(),
            glib_get_property: attrs.get_string("glib:get-property").ok(),
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
            ele => Err(ParseError::UnexpectedElement(Self::KIND, ele.kind())),
        }
    }
}

impl super::Element for MethodInline {
    const KIND: &'static str = "method-inline";

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
            ele => Err(ParseError::UnexpectedElement(Self::KIND, ele.kind())),
        }
    }
}

impl super::Element for VirtualMethod {
    const KIND: &'static str = "virtual-method";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            invoker: attrs.get_string("invoker").ok(),
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
            ele => Err(ParseError::UnexpectedElement(Self::KIND, ele.kind())),
        }
    }
}
