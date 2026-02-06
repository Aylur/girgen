use super::{AnyElement, Attrs, DocElement, ParseError};

pub struct Type {
    pub name: Option<String>,
    pub c_type: Option<String>,
    pub introspectable: Option<bool>,

    pub doc_elements: Vec<DocElement>,
    pub elements: Vec<AnyType>,
}

pub struct Array {
    pub name: Option<String>,
    pub c_type: Option<String>,
    pub zero_terminated: Option<bool>,
    pub fixed_size: Option<i32>,
    pub introspectable: Option<bool>,
    pub length: Option<i32>,

    pub elements: Vec<AnyType>,
}

pub enum AnyType {
    Type(Type),
    Array(Array),
}

impl AnyType {
    #[allow(clippy::result_large_err)]
    pub fn try_from_element(ele: AnyElement) -> Result<Self, AnyElement> {
        match ele {
            AnyElement::Array(a) => Ok(AnyType::Array(a)),
            AnyElement::Type(t) => Ok(AnyType::Type(t)),
            e => Err(e),
        }
    }
}

impl super::Element for Type {
    const KIND: &'static str = "type";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name").ok(),
            c_type: attrs.get_string("c:type").ok(),
            introspectable: attrs.get_boolean("introspectable").ok(),
            doc_elements: Vec::new(),
            elements: Vec::new(),
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

        match AnyType::try_from_element(element) {
            Ok(ok) => {
                self.elements.push(ok);
                Ok(())
            }
            Err(ele) => Err(ParseError::UnhandledXmlTag(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}

impl super::Element for Array {
    const KIND: &'static str = "array";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name").ok(),
            c_type: attrs.get_string("c:type").ok(),
            introspectable: attrs.get_boolean("introspectable").ok(),
            zero_terminated: attrs.get_boolean("zero-terminated").ok(),
            fixed_size: attrs.get_int("fixed-size").ok(),
            length: attrs.get_int("length").ok(),
            elements: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        match AnyType::try_from_element(element) {
            Ok(ok) => {
                self.elements.push(ok);
                Ok(())
            }
            Err(ele) => Err(ParseError::UnhandledXmlTag(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}
