use super::super::render;
use super::callable;
use crate::element;

impl render::Renderable for element::Function {
    const KIND: &'static str = "function";
    const TEMPLATE: &'static str = "{{ function }}";

    fn name(&self, _: &render::Context) -> &str {
        &self.attrs.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.return_value
            .as_ref()
            .is_none_or(|r| r.introspectable.is_none_or(|i| i))
            && self.parameters.as_ref().is_none_or(|ps| {
                ps.parameters
                    .iter()
                    .all(|p| p.introspectable.is_none_or(|i| i) && p.varargs.is_none())
            })
            && self.attrs.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, _: &render::Context) -> Result<minijinja::Value, String> {
        let args = callable::CallableArgs {
            info_elements: &self.info_elements,
            info: &self.attrs.info,
            throws: self.attrs.throws,
            prefix: Some("function "),
            name: Some(match &self.attrs.shadows {
                Some(name) => name,
                None => &self.attrs.name,
            }),
            parameters: self.parameters.as_ref(),
            returns: self.return_value.as_ref(),
        };

        Ok(minijinja::context! {
            function => callable::render(&args)?,
        })
    }
}
