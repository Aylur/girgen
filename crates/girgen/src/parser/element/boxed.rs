use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Boxed {
    pub info: InfoAttrs,
    pub glib_name: String,
    pub c_symbol_prefix: Option<String>,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,

    pub info_elements: Vec<InfoElement>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
}

impl super::Element for Boxed {
    const KIND: &'static str = "boxed";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            glib_name: attrs.get_string("glib:name")?,
            c_symbol_prefix: attrs.get_string("c:symbol-prefix").ok(),
            glib_type_name: attrs.get_string("glib:type-name").ok(),
            glib_get_type: attrs.get_string("glib:get-type").ok(),
            info_elements: Vec::new(),
            functions: Vec::new(),
            inline_functions: Vec::new(),
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
            AnyElement::Function(function) => {
                self.functions.push(function);
            }
            AnyElement::FunctionInline(function_inline) => {
                self.inline_functions.push(function_inline);
            }
            ele => {
                return Err(ParseError::UnexpectedElement(Self::KIND, ele.kind()));
            }
        }

        Ok(())
    }
}
