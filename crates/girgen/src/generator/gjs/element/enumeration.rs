use super::super::render;
use super::{callable, doc};
use crate::{element, generator::Event};
use std::sync::LazyLock;

static TEMPLATE: LazyLock<render::Template> = LazyLock::new(|| {
    render::compile_template(
        "enumeration.jinja",
        include_str!("../templates/enumeration.jinja"),
    )
});

macro_rules! render_functions {
    ($self:expr, $ctx:expr) => {
        $self
            .functions
            .iter()
            .filter_map(|f| {
                if !f.introspectable($ctx) {
                    return None;
                }

                let prefix = format!("{}: ", f.attrs.name);

                let args = callable::CallableArgs {
                    info_elements: &f.info_elements,
                    info: &f.attrs.info,
                    throws: f.attrs.throws,
                    prefix: Some(&prefix),
                    name: None,
                    parameters: f.parameters.as_ref(),
                    returns: f.return_value.as_ref(),
                };

                match callable::render($ctx, &args) {
                    Ok(res) => Some(res),
                    Err(err) => {
                        ($ctx.event)(Event::Failed {
                            repo: None,
                            err: &format!(
                                "failed to render {}-{}.{}.{} function: {}",
                                $ctx.namespace.name,
                                $ctx.namespace.version,
                                $self.name,
                                f.attrs.name,
                                err
                            ),
                        });
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
    };
}

macro_rules! render_members {
    ($self:expr, $ctx:expr) => {
        $self
            .members
            .iter()
            .map(|m| MemberContext {
                jsdoc: doc::jsdoc(&m.info_elements, &m.info).unwrap(),
                name: m.name.to_uppercase(),
                value: m.value.clone(),
            })
            .collect::<Vec<_>>()
    };
}

#[derive(serde::Serialize)]
pub struct MemberContext {
    jsdoc: String,
    name: String,
    value: String,
}

#[derive(serde::Serialize)]
pub struct EnumContext {
    suffix: &'static str,
    name: String,
    error_domain: Option<String>,
    jsdoc: String,
    functions: Vec<String>,
    members: Vec<MemberContext>,
}

impl render::Renderable<EnumContext> for element::Enumeration {
    const KIND: &'static str = "enum";
    const TEMPLATE: &'static LazyLock<render::Template> = &TEMPLATE;

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<EnumContext, String> {
        Ok(EnumContext {
            suffix: "Enum",
            name: self.name.clone(),
            error_domain: self.glib_error_domain.clone(),
            jsdoc: doc::jsdoc(&self.info_elements, &self.info)?,
            functions: render_functions!(self, ctx),
            members: render_members!(self, ctx),
        })
    }
}

impl render::Renderable<EnumContext> for element::Bitfield {
    const KIND: &'static str = "bitfield";
    const TEMPLATE: &'static LazyLock<render::Template> = &TEMPLATE;

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<EnumContext, String> {
        Ok(EnumContext {
            suffix: "Bitfield",
            name: self.name.clone(),
            error_domain: None,
            jsdoc: doc::jsdoc(&self.info_elements, &self.info)?,
            functions: render_functions!(self, ctx),
            members: render_members!(self, ctx),
        })
    }
}
