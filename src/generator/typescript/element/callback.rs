use super::super::render;
use super::callable;
use crate::element;

#[derive(serde::Serialize)]
pub struct CallbackContext {
    callback: String,
}

impl render::Renderable<CallbackContext> for element::Callback {
    const KIND: &'static str = "callback";
    const TEMPLATE: &'static str = "{{ callback }}";

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.return_value
            .as_ref()
            .is_none_or(|r| r.introspectable.is_none_or(|i| i))
            && self.parameter.as_ref().is_none_or(|ps| {
                ps.parameters
                    .iter()
                    .all(|p| p.introspectable.is_none_or(|i| i) && p.varargs.is_none())
            })
            && self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<CallbackContext, String> {
        let args = callable::CallableArgs {
            info_elements: &self.info_elements,
            info: &self.info,
            throws: self.throws,
            prefix: Some(&format!("type {} = ", self.name)),
            name: None,
            parameters: self.parameter.as_ref(),
            returns: self.return_value.as_ref(),
        };

        Ok(CallbackContext {
            callback: callable::render(ctx, &args)?,
        })
    }
}
