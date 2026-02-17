use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Record {
    pub info: InfoAttrs,
    pub name: String,
    pub c_type: Option<String>,
    pub disguised: Option<bool>,
    pub opaque: Option<bool>,
    pub pointer: Option<bool>,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,
    pub c_symbol_prefix: Option<String>,
    pub foreign: Option<bool>,
    pub glib_is_gtype_struct_for: Option<String>,
    pub copy_function: Option<String>,
    pub free_function: Option<String>,

    pub info_elements: Vec<InfoElement>,
    pub fields: Vec<super::Field>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
    pub unions: Vec<super::Union>,
    pub methods: Vec<super::Method>,
    pub inline_methods: Vec<super::MethodInline>,
    pub constructors: Vec<super::Constructor>,
}

impl super::Element for Record {
    const KIND: &'static str = "record";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            c_type: attrs.get_string("c:type").ok(),
            disguised: attrs.get_boolean("disguised").ok(),
            opaque: attrs.get_boolean("opaque").ok(),
            pointer: attrs.get_boolean("pointer").ok(),
            glib_type_name: attrs.get_string("glib:type-name").ok(),
            glib_get_type: attrs.get_string("glib:get-type").ok(),
            c_symbol_prefix: attrs.get_string("c:symbol-prefix").ok(),
            foreign: attrs.get_boolean("foreign").ok(),
            glib_is_gtype_struct_for: attrs.get_string("glib:is-gtype-struct-for").ok(),
            copy_function: attrs.get_string("copy-function").ok(),
            free_function: attrs.get_string("free-function").ok(),
            info_elements: Vec::new(),
            fields: Vec::new(),
            functions: Vec::new(),
            inline_functions: Vec::new(),
            unions: Vec::new(),
            methods: Vec::new(),
            inline_methods: Vec::new(),
            constructors: Vec::new(),
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
            AnyElement::Field(i) => self.fields.push(i),
            AnyElement::Function(i) => self.functions.push(i),
            AnyElement::FunctionInline(i) => self.inline_functions.push(i),
            AnyElement::Union(i) => self.unions.push(i),
            AnyElement::Method(i) => self.methods.push(i),
            AnyElement::MethodInline(i) => self.inline_methods.push(i),
            AnyElement::Constructor(i) => self.constructors.push(i),
            ele => {
                return Err(ParseError::UnexpectedElement(Self::KIND, ele.kind()));
            }
        }

        Ok(())
    }
}
