use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub info: InfoAttrs,

    pub name: String,
    pub c_type: String,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,
    pub glib_error_domain: Option<String>,

    pub info_elements: Vec<super::InfoElement>,
    pub members: Vec<super::Member>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
}

impl super::Element for Enumeration {
    const KIND: &'static str = "enumeration";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            c_type: attrs.get_string("c:type")?,
            glib_type_name: attrs.get_string("glib:type-name").ok(),
            glib_get_type: attrs.get_string("glib:get-type").ok(),
            glib_error_domain: attrs.get_string("glib:error-domain").ok(),
            info_elements: Vec::new(),
            members: Vec::new(),
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
            AnyElement::Member(m) => self.members.push(m),
            AnyElement::Function(f) => self.functions.push(f),
            AnyElement::FunctionInline(f) => self.inline_functions.push(f),
            ele => {
                return Err(ParseError::UnexpectedElement(Self::KIND, ele.kind()));
            }
        }

        Ok(())
    }
}
