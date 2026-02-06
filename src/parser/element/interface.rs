use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

pub struct Interface {
    pub info: InfoAttrs,
    pub name: String,
    pub glib_type_name: String,
    pub glib_get_type: String,
    pub c_symbol_prefix: Option<String>,
    pub c_type: Option<String>,
    pub glib_type_struct: Option<String>,

    pub info_elements: Vec<InfoElement>,
    pub prerequisites: Vec<Prerequisite>,
    pub implements: Vec<super::Implements>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
    pub constructors: Vec<super::Constructor>,
    pub methods: Vec<super::Method>,
    pub inline_methods: Vec<super::MethodInline>,
    pub virtual_methods: Vec<super::VirtualMethod>,
    pub fields: Vec<super::Field>,
    pub properties: Vec<super::Property>,
    pub signals: Vec<super::Signal>,
    pub callbacks: Vec<super::Callback>,
    pub constants: Vec<super::Constant>,
}

pub struct Prerequisite {
    pub name: String,
}

impl super::Element for Interface {
    const KIND: &'static str = "interface";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            glib_type_name: attrs.get_string("glib:type-name")?,
            glib_get_type: attrs.get_string("glib:get-type")?,
            c_symbol_prefix: attrs.get_string("c:symbol-prefix").ok(),
            c_type: attrs.get_string("c:type").ok(),
            glib_type_struct: attrs.get_string("glib:type-struct").ok(),
            info_elements: Vec::new(),
            prerequisites: Vec::new(),
            implements: Vec::new(),
            functions: Vec::new(),
            inline_functions: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
            inline_methods: Vec::new(),
            virtual_methods: Vec::new(),
            fields: Vec::new(),
            properties: Vec::new(),
            signals: Vec::new(),
            callbacks: Vec::new(),
            constants: Vec::new(),
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
            AnyElement::Prerequisite(r) => self.prerequisites.push(r),
            AnyElement::Implements(i) => self.implements.push(i),
            AnyElement::Constructor(c) => self.constructors.push(c),
            AnyElement::Method(m) => self.methods.push(m),
            AnyElement::MethodInline(m) => self.inline_methods.push(m),
            AnyElement::Function(f) => self.functions.push(f),
            AnyElement::FunctionInline(f) => self.inline_functions.push(f),
            AnyElement::VirtualMethod(m) => self.virtual_methods.push(m),
            AnyElement::Field(f) => self.fields.push(f),
            AnyElement::Property(p) => self.properties.push(p),
            AnyElement::Signal(s) => self.signals.push(s),
            AnyElement::Constant(c) => self.constants.push(c),
            AnyElement::Callback(c) => self.callbacks.push(c),
            ele => {
                return Err(ParseError::UnexpectedElement(format!(
                    "{}:{}",
                    Self::KIND,
                    ele.kind()
                )));
            }
        }

        Ok(())
    }
}

impl super::Element for Prerequisite {
    const KIND: &'static str = "prerequisites";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
        })
    }
}
