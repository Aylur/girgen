use super::super::render;
use super::gtype;
use crate::element;

#[derive(serde::Serialize)]
pub struct ConstantContext {
    name: String,
    value: String,
}

impl render::Renderable<ConstantContext> for element::Constant {
    const KIND: &'static str = "constant";
    const TEMPLATE: &'static str = "{{ name }}: {{ value }}";

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i) && self.name.parse::<i64>().is_err()
    }

    fn ctx(&self, _: &render::Context) -> Result<ConstantContext, String> {
        let value = gtype::tstype(self.r#type.as_ref(), false)?;

        let value = match value.as_str() {
            "boolean" | "number" => self.value.clone(),
            "string" => serde_json::to_string(&self.value)
                .expect("serializing a string constant should not fail"),
            _ => value,
        };

        let name = match self.name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            true => format!("\"{}\"", self.name),
            false => self.name.clone(),
        };

        Ok(ConstantContext { name, value })
    }
}
