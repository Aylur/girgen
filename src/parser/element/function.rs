use super::{AnyElement, Attrs, CallableAttrs, DocElement, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Function {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
}

#[derive(Debug, Clone)]
pub struct FunctionInline {
    pub attrs: CallableAttrs,
    pub parameters: Option<super::Parameters>,
    pub return_value: Option<super::ReturnValue>,
    pub doc_elements: Vec<DocElement>,
}

#[derive(Debug, Clone)]
pub struct FunctionMacro {
    pub attrs: CallableAttrs,
    pub info_elements: Vec<InfoElement>,
    pub parameters: Option<super::Parameters>,
}

impl super::Element for Function {
    const KIND: &'static str = "function";

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

impl super::Element for FunctionInline {
    const KIND: &'static str = "function-inline";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            parameters: None,
            return_value: None,
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

impl super::Element for FunctionMacro {
    const KIND: &'static str = "function-macro";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: CallableAttrs::new(attrs)?,
            info_elements: Vec::new(),
            parameters: None,
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
            ele => Err(ParseError::UnexpectedElement(Self::KIND, ele.kind())),
        }
    }
}
