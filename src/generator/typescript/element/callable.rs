use super::super::render;
use super::{doc, gtype};
use crate::element;
use crate::generator::Event;

static TEMPLATE: &str = include_str!("../templates/callable.jinja");

#[derive(serde::Serialize)]
struct Parameter<'a> {
    name: &'a str,
    tstype: String,
}

// https://gitlab.gnome.org/GNOME/gjs/-/blob/master/doc/Mapping.md#gtype-objects
fn override_parameter_type(name: &str) -> String {
    match name {
        "GObject.GType" => "(GObject.GType | { $gtype: GObject.GType })".to_owned(),
        _ => name.to_owned(),
    }
}

fn filter_keyword<'a>(name: &'a str) -> &'a str {
    match name {
        "in" => "in_",
        "break" => "break_",
        "function" => "func",
        _ => name,
    }
}

pub struct CallableArgs<'a> {
    pub info_elements: &'a [element::InfoElement],
    pub info: &'a element::InfoAttrs,
    pub throws: Option<bool>,
    pub overrides: bool,
    pub prefix: Option<&'a str>,
    pub name: Option<&'a str>,
    pub parameters: Option<&'a element::Parameters>,
    pub returns: Option<&'a element::ReturnValue>,
}

pub fn render(args: &CallableArgs) -> Result<String, String> {
    todo!()
}

macro_rules! callable_args {
    ($callable:expr, $prefix:expr) => {
        CallableArgs {
            info_elements: &$callable.info_elements,
            info: &$callable.attrs.info,
            throws: $callable.attrs.throws,
            overrides: $callable.attrs.shadows.is_some(),
            prefix: Some($prefix),
            name: Some(&$callable.attrs.name),
            parameters: $callable.parameters.as_ref(),
            returns: $callable.return_value.as_ref(),
        }
    };
}

pub enum CallableElement<'a> {
    Constructor(&'a element::Constructor),
    Function(&'a element::Function),
    Method(&'a element::Method),
    VirtualMethod(&'a element::VirtualMethod),
}

pub fn render_callable_elements(
    ctx: &render::Context,
    prefix: &str,
    elements: &[CallableElement<'_>],
) -> Vec<String> {
    elements
        .iter()
        .filter(|i| match i {
            CallableElement::Constructor(i) => i.attrs.info.introspectable.is_none_or(|i| i),
            CallableElement::Function(i) => i.attrs.info.introspectable.is_none_or(|i| i),
            CallableElement::Method(i) => i.attrs.info.introspectable.is_none_or(|i| i),
            CallableElement::VirtualMethod(i) => i.attrs.info.introspectable.is_none_or(|i| i),
        })
        .filter_map(|i| {
            let args = match i {
                CallableElement::Constructor(i) => callable_args!(i, prefix),
                CallableElement::Function(i) => callable_args!(i, prefix),
                CallableElement::Method(i) => callable_args!(i, prefix),
                CallableElement::VirtualMethod(i) => callable_args!(i, prefix),
            };

            let kind = match i {
                CallableElement::Constructor(_) => "constructor",
                CallableElement::Function(_) => "function",
                CallableElement::Method(_) => "method",
                CallableElement::VirtualMethod(_) => "virtual method",
            };

            let name = match i {
                CallableElement::Constructor(i) => &i.attrs.name,
                CallableElement::Function(i) => &i.attrs.name,
                CallableElement::Method(i) => &i.attrs.name,
                CallableElement::VirtualMethod(i) => &i.attrs.name,
            };

            match render(&args) {
                Ok(res) => Some(res),
                Err(err) => {
                    (ctx.event)(Event::Failed {
                        repo: None,
                        err: &format!(
                            "failed to render {} {}-{}.{}: {}",
                            kind, ctx.namespace.name, ctx.namespace.version, name, err
                        ),
                    });
                    None
                }
            }
        })
        .collect()
}
