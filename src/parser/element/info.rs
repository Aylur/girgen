use super::{AnyElement, Attribute, Attrs, DocElement};

pub struct InfoAttrs {
    pub introspectable: Option<bool>,
    pub deprecated: Option<bool>,
    pub deprecated_version: Option<String>,
    pub version: Option<String>,
    pub stability: Option<String>, // "Stable" | "Unstable" | "Private"
}

pub enum InfoElement {
    Annotation(Attribute),
    DocElement(DocElement),
}

impl InfoElement {
    #[allow(clippy::result_large_err)]
    pub fn try_from_element(element: AnyElement) -> Result<Self, AnyElement> {
        match DocElement::try_from_element(element) {
            Ok(doc) => Ok(InfoElement::DocElement(doc)),
            Err(AnyElement::Attribute(attr)) => Ok(InfoElement::Annotation(attr)),
            Err(ele) => Err(ele),
        }
    }
}

impl InfoAttrs {
    pub fn new(attrs: &Attrs) -> Self {
        Self {
            introspectable: attrs.get_boolean("introspectable").ok(),
            deprecated: attrs.get_boolean("deprecated").ok(),
            deprecated_version: attrs.get_string("deprecated-version").ok(),
            version: attrs.get_string("version").ok(),
            stability: attrs.get_string("stability").ok(),
        }
    }
}
