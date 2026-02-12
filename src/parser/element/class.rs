use super::{AnyElement, Attrs, InfoAttrs, InfoElement, ParseError};

#[derive(Debug, Clone)]
pub struct Class {
    pub info: InfoAttrs,
    pub name: String,
    pub glib_type_name: String,
    pub glib_get_type: String,
    pub parent: Option<String>,
    pub glib_type_struct: Option<String>,
    pub glib_ref_func: Option<String>,
    pub glib_unref_func: Option<String>,
    pub glib_set_value_func: Option<String>,
    pub glib_get_value_func: Option<String>,
    pub c_type: Option<String>,
    pub c_symbol_prefix: Option<String>,
    pub r#abstract: Option<bool>,
    pub r#final: Option<bool>,
    pub glib_fundamental: Option<bool>,

    pub info_elements: Vec<super::InfoElement>,
    pub implements: Vec<Implements>,
    pub constructors: Vec<super::Constructor>,
    pub methods: Vec<super::Method>,
    pub inline_methods: Vec<super::MethodInline>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
    pub virtual_methods: Vec<super::VirtualMethod>,
    pub fields: Vec<super::Field>,
    pub properties: Vec<super::Property>,
    pub signals: Vec<super::Signal>,
    pub unions: Vec<super::Union>,
    pub constants: Vec<super::Constant>,
    pub records: Vec<super::Record>,
    pub callbacks: Vec<super::Callback>,
}

#[derive(Debug, Clone)]
pub struct Implements {
    pub name: String,
}

impl super::Element for Class {
    const KIND: &'static str = "class";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            glib_type_name: attrs.get_string("glib:type-name")?,
            glib_get_type: attrs.get_string("glib:get-type")?,
            parent: attrs.get_string("parent").ok(),
            glib_type_struct: attrs.get_string("glib:type-struct").ok(),
            glib_ref_func: attrs.get_string("glib:ref-func").ok(),
            glib_unref_func: attrs.get_string("glib:unref-func").ok(),
            glib_set_value_func: attrs.get_string("glib:set-value-func").ok(),
            glib_get_value_func: attrs.get_string("glib:get-value-func").ok(),
            c_type: attrs.get_string("c:type").ok(),
            c_symbol_prefix: attrs.get_string("c:symbol-prefix").ok(),
            r#abstract: attrs.get_boolean("abstract").ok(),
            r#final: attrs.get_boolean("final").ok(),
            glib_fundamental: attrs.get_boolean("glib:fundamental").ok(),
            info_elements: Vec::new(),
            implements: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
            inline_methods: Vec::new(),
            functions: Vec::new(),
            inline_functions: Vec::new(),
            virtual_methods: Vec::new(),
            fields: Vec::new(),
            properties: Vec::new(),
            signals: Vec::new(),
            unions: Vec::new(),
            constants: Vec::new(),
            records: Vec::new(),
            callbacks: Vec::new(),
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
            AnyElement::Union(u) => self.unions.push(u),
            AnyElement::Constant(c) => self.constants.push(c),
            AnyElement::Record(r) => self.records.push(r),
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

impl super::Element for Implements {
    const KIND: &'static str = "implements";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
        })
    }
}
