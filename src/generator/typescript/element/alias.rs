use super::super::render;
use super::{doc, gtype};
use crate::element;

#[derive(serde::Serialize)]
pub struct AliasContext {
    jsdoc: String,
    name: String,
    value: String,
}

impl render::Renderable<AliasContext> for element::Alias {
    const KIND: &'static str = "alias";
    const TEMPLATE: &'static str = "{{ jsdoc if jsdoc }}\ntype {{ name }} = {{ value }}";

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _ctx: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, _: &render::Context) -> Result<AliasContext, String> {
        let value = gtype::tstype(self.r#type.as_ref(), false)?;
        let jsdoc = doc::jsdoc(&self.info_elements, &self.info).unwrap();

        Ok(AliasContext {
            jsdoc,
            name: self.name.clone(),
            value,
        })
    }
}
