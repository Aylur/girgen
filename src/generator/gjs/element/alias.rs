use super::super::render;
use super::{doc, gtype};
use crate::element;
use std::sync::LazyLock;

static TEMPLATE: LazyLock<render::Template> = LazyLock::new(|| {
    render::compile_template(
        "alias",
        "{{ jsdoc if jsdoc }}\ntype {{ name|escape_toplevel }} = {{ value }}",
    )
});

#[derive(serde::Serialize)]
pub struct AliasContext {
    jsdoc: String,
    name: String,
    value: String,
}

impl render::Renderable<AliasContext> for element::Alias {
    const KIND: &'static str = "alias";
    const TEMPLATE: &'static LazyLock<render::Template> = &TEMPLATE;

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
