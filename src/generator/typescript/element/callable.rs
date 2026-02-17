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

fn override_parameter_type(name: &str) -> &str {
    match name {
        "GObject.Value" => "(GObject.Value | unknown)",
        "GObject.GType" => "(GObject.GType | { $gtype: GObject.GType })",
        _ => name,
    }
}

fn override_return_type(name: &str) -> &str {
    match name {
        "GObject.Value" => "unknown",
        _ => name,
    }
}

fn filter_keyword(name: Option<&str>) -> &str {
    match name {
        None => "arg",
        Some("in") => "in_",
        Some("break") => "break_",
        Some("function") => "func",
        Some(name) => name,
    }
}

pub struct CallableArgs<'a> {
    pub info_elements: &'a [element::InfoElement],
    pub info: &'a element::InfoAttrs,
    pub throws: Option<bool>,
    pub prefix: Option<&'a str>,
    pub name: Option<&'a str>,
    pub parameters: Option<&'a element::Parameters>,
    pub returns: Option<&'a element::ReturnValue>,
}

pub fn render(_: &render::Context, args: &CallableArgs) -> Result<String, String> {
    let env = minijinja::Environment::new();

    let (p_returns, p_parameters): (Vec<_>, Vec<_>) =
        gtype::filter_parameters(args.parameters, args.returns)
            .into_iter()
            .partition(|p| matches!(p.direction.as_deref(), Some("inout" | "out")));

    let parameter_results: Vec<Result<Parameter, String>> = p_parameters
        .into_iter()
        .map(|p| -> Result<Parameter, String> {
            let t = gtype::tstype(p.r#type.as_ref(), p.nullable.is_some_and(|n| n))?;
            Ok(Parameter {
                name: filter_keyword(p.name.as_deref()),
                tstype: override_parameter_type(&t).to_owned(),
            })
        })
        .collect();

    let return_results: Vec<Result<String, String>> = args
        .returns
        .filter(|r| {
            r.r#type.as_ref().is_some_and(|t| {
                matches!(t, element::AnyType::Type(t) if t.name.as_deref() != Some("none"))
                    || matches!(t, element::AnyType::Array(_))
            })
        })
        .into_iter()
        .map(|r| gtype::tstype(r.r#type.as_ref(), r.nullable.is_some_and(|n| n)))
        .chain(
            p_returns
                .into_iter()
                .filter(|p| p.optional.is_none_or(|o| !o))
                .map(|p| {
                    gtype::tstype(p.r#type.as_ref(), p.nullable.is_some_and(|n| n))
                        .map(|t| override_return_type(&t).to_owned())
                }),
        )
        .collect();

    let return_errs = return_results.iter().filter_map(|r| {
        r.as_ref()
            .err()
            .map(|err| format!("failed to render callable 'return': {}", err))
    });

    let param_errs = parameter_results.iter().filter_map(|p| {
        p.as_ref()
            .err()
            .map(|err| format!("failed to render callable 'parameter': {}", err))
    });

    let errs: Vec<String> = return_errs.chain(param_errs).collect();

    if !errs.is_empty() {
        return Err(errs.join(","));
    }

    let returns: Vec<String> = return_results.into_iter().map(|r| r.unwrap()).collect();
    let parameters: Vec<Parameter> = parameter_results.into_iter().map(|p| p.unwrap()).collect();

    let jsdoc = doc::jsdoc_with_args(&doc::DocArgs {
        info_elements: args.info_elements,
        info: args.info,
        parameters: args.parameters,
        returns: args.returns,
        throws: args.throws,
        default_value: None,
    })?;

    let name = args.name.map(|name| match name {
        "new" => "\"new\"",
        n => n,
    });

    let ctx = minijinja::context! {
        jsdoc,
        prefix => args.prefix,
        name,
        parameters,
        returns,
    };

    match env.render_str(TEMPLATE, ctx) {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("failed to render callable: {:?}", err)),
    }
}

macro_rules! callable_args {
    ($callable:expr, $prefix:expr, $name:expr) => {{
        CallableArgs {
            info_elements: &$callable.info_elements,
            info: &$callable.attrs.info,
            throws: $callable.attrs.throws,
            prefix: Some($prefix),
            name: Some($name),
            parameters: $callable.parameters.as_ref(),
            returns: $callable.return_value.as_ref(),
        }
    }};
}

macro_rules! callable_name {
    ($callable:expr) => {{
        match &$callable.attrs.shadows {
            Some(name) => name,
            None => &$callable.attrs.name,
        }
    }};
}

macro_rules! callable_filter {
    ($callable:expr) => {{
        match &$callable.attrs.shadows {
            Some(_) => true,
            None => match &$callable.attrs.shadowed_by {
                Some(_) => false,
                None => $callable.attrs.info.introspectable.is_none_or(|i| i),
            },
        }
    }};
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
            CallableElement::Constructor(i) => callable_filter!(i),
            CallableElement::Function(i) => callable_filter!(i),
            CallableElement::Method(i) => callable_filter!(i),
            CallableElement::VirtualMethod(i) => callable_filter!(i),
        })
        .filter_map(|i| {
            let kind = match i {
                CallableElement::Constructor(_) => "constructor",
                CallableElement::Function(_) => "function",
                CallableElement::Method(_) => "method",
                CallableElement::VirtualMethod(_) => "virtual method",
            };

            let name = match i {
                CallableElement::Constructor(i) => callable_name!(i),
                CallableElement::Function(i) => callable_name!(i),
                CallableElement::Method(i) => callable_name!(i),
                CallableElement::VirtualMethod(i) => callable_name!(i),
            };

            let args = match i {
                CallableElement::Constructor(i) => callable_args!(i, prefix, name),
                CallableElement::Function(i) => callable_args!(i, prefix, name),
                CallableElement::Method(i) => callable_args!(i, prefix, name),
                CallableElement::VirtualMethod(i) => callable_args!(i, prefix, name),
            };

            match render(ctx, &args) {
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
