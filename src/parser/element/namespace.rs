use super::{AnyElement, Attrs, ParseError};

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub version: String,
    pub c_symbol_prefixes: Option<String>,
    pub aliases: Vec<super::Alias>,
    pub classes: Vec<super::Class>,
    pub interfaces: Vec<super::Interface>,
    pub records: Vec<super::Record>,
    pub enums: Vec<super::Enumeration>,
    pub functions: Vec<super::Function>,
    pub inline_functions: Vec<super::FunctionInline>,
    pub macro_functions: Vec<super::FunctionMacro>,
    pub unions: Vec<super::Union>,
    pub bitfields: Vec<super::Bitfield>,
    pub callbacks: Vec<super::Callback>,
    pub constants: Vec<super::Constant>,
    pub annotations: Vec<super::Attribute>,
    pub boxeds: Vec<super::Boxed>,
    pub doc_sections: Vec<super::DocSection>,
}

impl super::Element for Namespace {
    const KIND: &'static str = "namespace";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
            version: attrs.get_string("version")?,
            c_symbol_prefixes: attrs.get_string("c:symbol-prefixes").ok(),
            aliases: Vec::new(),
            classes: Vec::new(),
            interfaces: Vec::new(),
            records: Vec::new(),
            enums: Vec::new(),
            functions: Vec::new(),
            inline_functions: Vec::new(),
            macro_functions: Vec::new(),
            unions: Vec::new(),
            bitfields: Vec::new(),
            callbacks: Vec::new(),
            constants: Vec::new(),
            annotations: Vec::new(),
            boxeds: Vec::new(),
            doc_sections: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        match element {
            AnyElement::Alias(i) => self.aliases.push(i),
            AnyElement::Class(i) => self.classes.push(i),
            AnyElement::Interface(i) => self.interfaces.push(i),
            AnyElement::Record(i) => self.records.push(i),
            AnyElement::Enumeration(i) => self.enums.push(i),
            AnyElement::Function(i) => self.functions.push(i),
            AnyElement::FunctionInline(i) => self.inline_functions.push(i),
            AnyElement::FunctionMacro(i) => self.macro_functions.push(i),
            AnyElement::Union(i) => self.unions.push(i),
            AnyElement::Bitfield(i) => self.bitfields.push(i),
            AnyElement::Callback(i) => self.callbacks.push(i),
            AnyElement::Constant(i) => self.constants.push(i),
            AnyElement::Attribute(i) => self.annotations.push(i),
            AnyElement::Boxed(i) => self.boxeds.push(i),
            AnyElement::DocSection(i) => self.doc_sections.push(i),
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
