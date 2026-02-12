use super::super::render;
use super::{callable, doc};
use crate::{element, generator::Event};

const TEMPLATE: &str = include_str!("../templates/enumeration.jinja");

fn remove_prefix(fn_name: &str, prefixes: &Vec<&str>) -> String {
    for prefix in prefixes {
        if let Some(name) = fn_name.strip_prefix(prefix) {
            return name.to_string();
        }
    }
    fn_name.to_string()
}

macro_rules! render_functions {
    ($self:expr, $ctx:expr) => {
        if $self.functions.is_empty() {
            Vec::new()
        } else {
            match $ctx.namespace.c_symbol_prefixes.as_ref() {
                None => {
                    ($ctx.event)(Event::Failed {
                        repo: None,
                        err: &format!(
                            "failed to render {}-{}.{} functions: Missing namespace c:symbol-prefix",
                            $ctx.namespace.name,
                            $ctx.namespace.version,
                            $self.name
                        ),
                    });
                    Vec::new()
                }
                Some(prefixes) => {
                    let ns_prefixes: Vec<_> = prefixes.split(',').collect();

                    $self
                        .functions
                        .iter()
                        .filter_map(|f| {
                            if !f.introspectable($ctx) {
                                return None;
                            }

                            let args = callable::CallableArgs {
                                info_elements: &f.info_elements,
                                info: &f.attrs.info,
                                throws: f.attrs.throws,
                                prefix: Some("function "),
                                name: Some(&remove_prefix(&f.attrs.name, &ns_prefixes)),
                                parameters: f.parameters.as_ref(),
                                returns: f.return_value.as_ref(),
                            };

                            match callable::render(&args) {
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
                                        )
                                    });
                                    None
                                }
                            }
                        })
                        .collect()
                }
            }
        }
    };
}

macro_rules! render_members {
    ($self:expr, $ctx:expr) => {
        $self
            .members
            .iter()
            .map(|m| {
                minijinja::context! {
                    jsdoc => doc::jsdoc(&m.info_elements, &m.info).unwrap(),
                    name => m.name.to_uppercase(),
                    value => m.value,
                }
            })
            .collect::<Vec<minijinja::Value>>()
    };
}

impl render::Renderable for element::Enumeration {
    const KIND: &'static str = "enum";
    const TEMPLATE: &'static str = TEMPLATE;

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<minijinja::Value, String> {
        let jsdoc = doc::jsdoc(&self.info_elements, &self.info)?;
        let functions = render_functions!(self, ctx);
        let members = render_members!(self, ctx);

        Ok(minijinja::context! {
            name => &self.name,
            error_domain => &self.glib_error_domain,
            jsdoc,
            functions,
            members,
        })
    }
}

impl render::Renderable for element::Bitfield {
    const KIND: &'static str = "bitfield";
    const TEMPLATE: &'static str = TEMPLATE;

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<minijinja::Value, String> {
        let jsdoc = doc::jsdoc(&self.info_elements, &self.info)?;
        let functions = render_functions!(self, ctx);
        let members = render_members!(self, ctx);

        Ok(minijinja::context! {
            name => &self.name,
            jsdoc,
            functions,
            members,
        })
    }
}
