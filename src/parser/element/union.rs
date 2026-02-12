use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Union {
    pub info: InfoAttrs,

    pub name: Option<String>,
    pub c_type: Option<String>,
    pub c_symbol_prefix: Option<String>,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,
    pub copy_function: Option<String>,
    pub free_function: Option<String>,

    pub info_elements: Vec<InfoElement>,
    pub fields: Vec<super::Field>,
    pub constructors: Vec<super::Constructor>,
    pub methods: Vec<super::Method>,
    pub inline_methods: Vec<super::MethodInline>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
    pub records: Vec<super::Record>,
}

impl super::Element for Union {
    const KIND: &'static str = "union";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name").ok(),
            c_type: attrs.get_string("c:type").ok(),
            c_symbol_prefix: attrs.get_string("c:symbol-prefix").ok(),
            glib_type_name: attrs.get_string("glib:type-name").ok(),
            glib_get_type: attrs.get_string("glib:get-type").ok(),
            copy_function: attrs.get_string("copy-function").ok(),
            free_function: attrs.get_string("free-function").ok(),
            info_elements: Vec::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
            inline_methods: Vec::new(),
            functions: Vec::new(),
            inline_functions: Vec::new(),
            records: Vec::new(),
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
            AnyElement::Constructor(i) => self.constructors.push(i),
            AnyElement::Method(i) => self.methods.push(i),
            AnyElement::MethodInline(i) => self.inline_methods.push(i),
            AnyElement::Function(i) => self.functions.push(i),
            AnyElement::FunctionInline(i) => self.inline_functions.push(i),
            AnyElement::Record(i) => self.records.push(i),
            ele => {
                return Err(ParseError::UnhandledXmlTag(format!(
                    "{}:{}",
                    Self::KIND,
                    ele.kind()
                )));
            }
        }

        Ok(())
    }
}
