use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Bitfield {
    pub info: InfoAttrs,

    pub name: String,
    pub c_type: String,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,

    pub info_elements: Vec<InfoElement>,
    pub members: Vec<super::Member>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
}

impl super::Element for Bitfield {
    const KIND: &'static str = "bitfield";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            c_type: attrs.get_string("c:type")?,
            glib_type_name: attrs.get_string("glib:type-name").ok(),
            glib_get_type: attrs.get_string("glib:get-type").ok(),
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
            AnyElement::Member(member) => {
                self.members.push(member);
            }
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
