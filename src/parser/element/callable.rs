use super::{AnyElement, AnyType, Attrs, DocElement, InfoAttrs, ParseError};

pub struct CallableAttrs {
    pub info: InfoAttrs,
    pub name: String,
    pub c_identifier: Option<String>,
    pub shadowed_by: Option<String>,
    pub shadows: Option<String>,
    pub throws: Option<bool>,
    pub moved_to: Option<String>,
    pub glib_async_func: Option<String>,
    pub glib_sync_func: Option<String>,
    pub glib_finish_func: Option<String>,
}

pub struct VarArgs;

pub struct Parameters {
    pub instance_parameter: Option<InstanceParameter>,
    pub parameters: Vec<Parameter>,
}

pub struct InstanceParameter {
    pub name: String,
    pub nullable: Option<bool>,
    pub allow_none: Option<bool>, // depreacted, replaced by nullable and optional
    pub direction: Option<String>, // "out" | "in" | "inout"
    pub caller_allocates: Option<bool>,

    pub r#type: Option<AnyType>,
    pub doc_elements: Vec<DocElement>,
}

pub struct Parameter {
    pub name: Option<String>,
    pub nullable: Option<bool>,
    pub allow_none: Option<bool>, // depreacted, replaced by nullable and optional
    pub introspectable: Option<bool>,
    pub closure: Option<i32>,
    pub destroy: Option<i32>,
    pub scope: Option<String>, // "notified" | "async" | "call" | "forever"
    pub direction: Option<String>, // "out" | "in" | "inout"
    pub caller_allocates: Option<bool>,
    pub optional: Option<bool>,
    pub skip: Option<bool>,

    pub doc_elements: Vec<DocElement>,
    pub varargs: Option<VarArgs>,
    pub r#type: Option<AnyType>,
    pub annotations: Vec<super::Attribute>,
}

pub struct ReturnValue {
    pub introspectable: Option<bool>,
    pub nullable: Option<bool>,
    pub closure: Option<i32>,
    pub scope: Option<String>, // "notified" | "async" | "call" | "forever"
    pub destroy: Option<i32>,
    pub skip: Option<bool>,
    pub allow_none: Option<bool>, // depreacted, replaced by nullable and optional

    pub doc_elements: Vec<DocElement>,
    pub annotations: Vec<super::Attribute>,
    pub r#type: Option<AnyType>,
}

impl CallableAttrs {
    pub fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            info: InfoAttrs::new(attrs),
            name: attrs.get_string("name")?,
            c_identifier: attrs.get_string("c:identifier").ok(),
            shadowed_by: attrs.get_string("shadowed-by").ok(),
            shadows: attrs.get_string("shadows").ok(),
            throws: attrs.get_boolean("throws").ok(),
            moved_to: attrs.get_string("moved-to").ok(),
            glib_async_func: attrs.get_string("glib:async-func").ok(),
            glib_sync_func: attrs.get_string("glib:sync-func").ok(),
            glib_finish_func: attrs.get_string("glib:finish-func").ok(),
        })
    }
}

impl super::Element for VarArgs {
    const KIND: &'static str = "varargs";

    fn new(_: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

impl super::Element for Parameters {
    const KIND: &'static str = "parameters";

    fn new(_: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            instance_parameter: None,
            parameters: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        match element {
            AnyElement::InstanceParameter(p) => self.instance_parameter = Some(p),
            AnyElement::Parameter(p) => {
                self.parameters.push(p);
            }
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

impl super::Element for InstanceParameter {
    const KIND: &'static str = "instance-parameter";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
            nullable: attrs.get_boolean("nullable").ok(),
            allow_none: attrs.get_boolean("allow-none").ok(),
            direction: attrs.get_string("direction").ok(),
            caller_allocates: attrs.get_boolean("caller-allocates").ok(),
            r#type: None,
            doc_elements: Vec::new(),
        })
    }

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        match DocElement::try_from_element(element) {
            Ok(ok) => {
                self.doc_elements.push(ok);
                Ok(())
            }
            Err(ele) => Err(ParseError::UnexpectedElement(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}

impl super::Element for Parameter {
    const KIND: &'static str = "parameter";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name").ok(),
            nullable: attrs.get_boolean("nullable").ok(),
            allow_none: attrs.get_boolean("allow-none").ok(),
            introspectable: attrs.get_boolean("introspectable").ok(),
            closure: attrs.get_int("closure").ok(),
            destroy: attrs.get_int("destroy").ok(),
            scope: attrs.get_string("scope").ok(),
            direction: attrs.get_string("direction").ok(),
            caller_allocates: attrs.get_boolean("caller-allocates").ok(),
            optional: attrs.get_boolean("optional").ok(),
            skip: attrs.get_boolean("skip").ok(),
            doc_elements: Vec::new(),
            varargs: None,
            r#type: None,
            annotations: Vec::new(),
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

        let element = match AnyType::try_from_element(element) {
            Err(ele) => ele,
            Ok(ok) => {
                self.r#type = Some(ok);
                return Ok(());
            }
        };

        match element {
            AnyElement::VarArgs(args) => {
                self.varargs = Some(args);
            }
            AnyElement::Attribute(attr) => {
                self.annotations.push(attr);
            }
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

impl super::Element for ReturnValue {
    const KIND: &'static str = "return-value";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            introspectable: attrs.get_boolean("introspectable").ok(),
            nullable: attrs.get_boolean("nullable").ok(),
            closure: attrs.get_int("closure").ok(),
            scope: attrs.get_string("scope").ok(),
            destroy: attrs.get_int("destroy").ok(),
            skip: attrs.get_boolean("skip").ok(),
            allow_none: attrs.get_boolean("allow-none").ok(),
            doc_elements: Vec::new(),
            annotations: Vec::new(),
            r#type: None,
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

        let element = match AnyType::try_from_element(element) {
            Err(ele) => ele,
            Ok(ok) => {
                self.r#type = Some(ok);
                return Ok(());
            }
        };

        match element {
            AnyElement::Attribute(attr) => {
                self.annotations.push(attr);
                Ok(())
            }
            ele => Err(ParseError::UnexpectedElement(format!(
                "{}:{}",
                Self::KIND,
                ele.kind()
            ))),
        }
    }
}
