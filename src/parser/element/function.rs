use super::{AnyElement, Attrs, CallableAttrs, DocElement, InfoElement, ParseError};

pub struct Function {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub params: Option<super::Parameters>,
    pub r#return: Option<super::ReturnValue>,
}

pub struct FunctionInline {
    pub attrs: CallableAttrs,
    pub params: Option<super::Parameters>,
    pub r#return: Option<super::ReturnValue>,
    pub doc_elements: Vec<DocElement>,
}

pub struct FunctionMacro {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub params: Option<super::Parameters>,
}

impl super::Element for Function {
    const KIND: &'static str = "function";

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

impl super::Element for FunctionInline {
    const KIND: &'static str = "function-inline";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            params: None,
            r#return: None,
            doc_elements: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        let element = match DocElement::try_from_element(element) {
            Err(ele) => ele,
            Ok(ok) => {
                self.doc_elements.push(ok);
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

impl super::Element for FunctionMacro {
    const KIND: &'static str = "function-macro";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            info_elements: Vec::new(),
            params: None,
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
            ele => Err(ParseError::UnexpectedElement(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}
