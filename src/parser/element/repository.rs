use super::{AnyElement, Attrs, ParseError};

#[derive(Debug, Clone)]
pub struct Repository {
    pub version: Option<String>,
    pub c_identifier_prefixes: Option<String>,
    pub c_symbol_prefixes: Option<String>,

    pub includes: Vec<Include>,
    pub c_includes: Vec<super::CInclude>,
    pub packages: Vec<super::Package>,
    pub namespaces: Vec<super::Namespace>,
    pub doc_formats: Vec<super::DocFormat>,
}

#[derive(Debug, Clone)]
pub struct Include {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct CInclude {
    pub name: String,
}

impl super::Element for CInclude {
    const KIND: &'static str = "c:include";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
        })
    }
}

impl super::Element for Include {
    const KIND: &'static str = "include";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Self {
            name: attrs.get_string("name")?,
            version: attrs.get_string("version")?,
        })
    }
}

impl super::Element for Repository {
    const KIND: &'static str = "repository";

    fn new(attrs: &Attrs) -> Result<Self, ParseError> {
        Ok(Repository {
            version: attrs.get_string("version").ok(),
            c_identifier_prefixes: attrs.get_string("c:identifier-prefixes").ok(),
            c_symbol_prefixes: attrs.get_string("c:symbol-prefixes").ok(),
            includes: Vec::new(),
            c_includes: Vec::new(),
            packages: Vec::new(),
            namespaces: Vec::new(),
            doc_formats: Vec::new(),
        })
    }

    fn end(&mut self, element: super::AnyElement) -> Result<(), ParseError> {
        match element {
            AnyElement::Include(include) => self.includes.push(include),
            AnyElement::CInclude(cinclude) => self.c_includes.push(cinclude),
            AnyElement::Package(package) => self.packages.push(package),
            AnyElement::Namespace(namespace) => self.namespaces.push(namespace),
            AnyElement::DocFormat(doc_format) => self.doc_formats.push(doc_format),
            ele => {
                return Err(ParseError::UnexpectedElement(Self::KIND, ele.kind()));
            }
        }
        Ok(())
    }
}

impl Repository {
    pub fn find_includes(&self, repos: &[&Repository]) -> Vec<Include> {
        let mut result = self
            .includes
            .iter()
            .filter_map(|i| {
                repos.iter().find(|r| {
                    r.namespaces
                        .iter()
                        .any(|ns| ns.name == i.name && ns.version == i.version)
                })
            })
            .flat_map(|r| r.find_includes(repos))
            .collect::<Vec<_>>();

        for inc in &self.includes {
            result.push(Include {
                name: inc.name.clone(),
                version: inc.version.clone(),
            });
        }

        let mut seen = std::collections::HashSet::new();
        result.retain(|inc| seen.insert((inc.name.clone(), inc.version.clone())));
        result
    }
}
