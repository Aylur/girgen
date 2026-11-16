use super::super::{overrides, render};
use super::{doc, gtype};
use crate::element;
use crate::generator::Event;
use minijinja::value::Serde;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static TEMPLATE: LazyLock<render::Template> = LazyLock::new(|| {
    render::compile_template(
        "callable.jinja",
        include_str!("../templates/callable.jinja"),
    )
});

#[derive(serde::Serialize)]
struct Parameter<'a> {
    name: &'a str,
    tstype: String,
}

// FIXME: override before tstype to avoid duplicated nullable case
fn override_parameter_type(name: &str) -> &str {
    match name {
        "GObject.Value" => "(GObject.Value | unknown)",
        "GObject.Closure" => "((...args: unknown[]) => unknown)",
        "GObject.Closure | null" => "((...args: unknown[]) => unknown) | null",
        "GObject.GType" => "(GObject.GType | { $gtype: GObject.GType })",
        "GLib.Bytes" => "(GLib.Bytes | Uint8Array)",
        "GLib.Bytes | null" => "(GLib.Bytes | Uint8Array | null)",
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

#[derive(Debug)]
pub struct CallableArgs<'a> {
    pub info_elements: &'a [element::InfoElement],
    pub info: &'a element::InfoAttrs,
    pub throws: Option<bool>,
    pub prefix: Option<&'a str>,
    pub name: Option<&'a str>,
    pub parameters: Option<&'a element::Parameters>,
    pub returns: Option<&'a element::ReturnValue>,
}

fn resolve_types<'a>(args: &CallableArgs<'a>) -> Result<(Vec<Parameter<'a>>, Vec<String>), String> {
    let params = gtype::filter_parameters(args.parameters, args.returns);

    let p_returns: Vec<_> = params
        .iter()
        .filter(|p| matches!(p.direction.as_deref(), Some("inout" | "out")))
        .cloned()
        .collect();

    let p_parameters: Vec<_> = params
        .iter()
        .filter(|p| matches!(p.direction.as_deref(), None | Some("in" | "inout")))
        .cloned()
        .collect();

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
            // returns non-void or an array
            r.r#type.as_ref().is_some_and(|t| {
                matches!(t, element::AnyType::Type(t) if t.name.as_deref() != Some("none"))
                    || matches!(t, element::AnyType::Array(_))
            })
        })
        .into_iter()
        .map(|r| gtype::tstype(r.r#type.as_ref(), r.nullable.is_some_and(|n| n)))
        .chain(p_returns.into_iter().map(|p| {
            gtype::tstype(p.r#type.as_ref(), p.nullable.is_some_and(|n| n))
                .map(|t| override_return_type(&t).to_owned())
        }))
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

    Ok((parameters, returns))
}

fn member_name(name: &str) -> String {
    match name {
        "" => "\"\"".to_owned(),
        "new" => "\"new\"".to_owned(),
        n if n.chars().next().is_some_and(|c| c.is_ascii_digit()) => format!("\"{n}\""),
        n => n.to_owned(),
    }
}

pub fn render(ctx: &render::Context, args: &CallableArgs) -> Result<String, String> {
    render_with_supertypes(ctx, args, &[])
}

/// `supertypes` are the signatures of members with the same name in supertypes.
/// When it conflicts with any of them, an additional `name(...args: never[]): any`
/// overload is rendered which is assignable to any signature.
fn render_with_supertypes(
    ctx: &render::Context,
    args: &CallableArgs,
    supertypes: &[Option<Signature>],
) -> Result<String, String> {
    let (parameters, returns) = resolve_types(args)?;

    let conflicting = !supertypes.is_empty() && {
        let own = Signature::new(&ctx.namespace.name, &parameters, &returns);
        supertypes.iter().any(|sup| match sup {
            Some(sup) => is_conflicting(ctx, &own, sup),
            // signature is unknown
            None => true,
        })
    };

    let jsdoc = doc::jsdoc_with_args(&doc::DocArgs {
        info_elements: args.info_elements,
        info: args.info,
        parameters: args.parameters,
        returns: args.returns,
        throws: args.throws,
        default_value: None,
    })?;

    let name = args.name.map(member_name);

    let ctx = minijinja::context! {
        jsdoc,
        prefix => args.prefix,
        name,
        parameters => Serde(&parameters),
        returns,
        conflicting,
    };

    match TEMPLATE.render(ctx) {
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
            None => $callable.attrs.info.introspectable.is_none_or(|i| i),
        }
    }};
}

pub enum CallableElement<'a> {
    Constructor(&'a element::Constructor),
    Function(&'a element::Function),
    Method(&'a element::Method),
    VirtualMethod(&'a element::VirtualMethod),
}

impl<'a> CallableElement<'a> {
    fn kind(&self) -> &'static str {
        match self {
            CallableElement::Constructor(_) => "constructor",
            CallableElement::Function(_) => "function",
            CallableElement::Method(_) => "method",
            CallableElement::VirtualMethod(_) => "virtual method",
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            CallableElement::Constructor(i) => callable_name!(i),
            CallableElement::Function(i) => callable_name!(i),
            CallableElement::Method(i) => callable_name!(i),
            CallableElement::VirtualMethod(i) => callable_name!(i),
        }
    }

    fn is_rendered(&self) -> bool {
        match self {
            CallableElement::Constructor(i) => callable_filter!(i),
            CallableElement::Function(i) => callable_filter!(i),
            CallableElement::Method(i) => callable_filter!(i),
            CallableElement::VirtualMethod(i) => callable_filter!(i),
        }
    }

    fn args<'b>(&self, prefix: &'b str) -> CallableArgs<'b>
    where
        'a: 'b,
    {
        let name = self.name();
        match self {
            CallableElement::Constructor(i) => callable_args!(i, prefix, name),
            CallableElement::Function(i) => callable_args!(i, prefix, name),
            CallableElement::Method(i) => callable_args!(i, prefix, name),
            CallableElement::VirtualMethod(i) => callable_args!(i, prefix, name),
        }
    }
}

pub fn render_callable_elements(
    ctx: &render::Context,
    prefix: &str,
    elements: &[CallableElement<'_>],
) -> Vec<String> {
    render_elements(ctx, prefix, elements, &HashMap::new())
}

/// Renders methods or virtual methods of the class or interface `owner`,
/// resolving conflicts with members of the same name in its supertypes.
pub fn render_member_elements(
    ctx: &render::Context,
    owner: &str,
    prefix: &str,
    elements: &[CallableElement<'_>],
) -> Vec<String> {
    render_elements(ctx, prefix, elements, &inherited_signatures(ctx, owner))
}

fn render_elements(
    ctx: &render::Context,
    prefix: &str,
    elements: &[CallableElement<'_>],
    inherited: &HashMap<String, Vec<Option<Signature>>>,
) -> Vec<String> {
    elements
        .iter()
        .filter(|i| i.is_rendered())
        .filter_map(|i| {
            let name = i.name();
            let supertypes = inherited
                .get(&format!("{prefix}{name}"))
                .map(Vec::as_slice)
                .unwrap_or_default();

            match render_with_supertypes(ctx, &i.args(prefix), supertypes) {
                Ok(res) => Some(res),
                Err(err) => {
                    (ctx.event)(Event::Failed {
                        repo: None,
                        err: &format!(
                            "failed to render {} {}-{}.{}: {}",
                            i.kind(),
                            ctx.namespace.name,
                            ctx.namespace.version,
                            name,
                            err
                        ),
                    });
                    None
                }
            }
        })
        .collect()
}

// Conflicts with supertypes
//
// TypeScript requires every member of a subtype to be assignable to the member
// of the same name in its supertypes, which GObject APIs don't respect,
// for example `Gtk.Container.child_notify(child, property)` and
// `Gtk.Widget.child_notify(property)`.
// Members inherited from multiple supertypes with different declarations
// (diamonds) are not resolved.

/// Parameter and return types with every type name qualified with its namespace.
struct Signature {
    parameters: Vec<String>,
    returns: Vec<String>,
}

impl Signature {
    fn new(namespace: &str, parameters: &[Parameter], returns: &[String]) -> Self {
        Self {
            parameters: parameters
                .iter()
                .map(|p| qualify(namespace, &p.tstype))
                .collect(),
            returns: returns.iter().map(|r| qualify(namespace, r)).collect(),
        }
    }
}

#[derive(Clone, Copy)]
enum Supertype<'a> {
    Class(&'a element::Namespace, &'a element::Class),
    Interface(&'a element::Namespace, &'a element::Interface),
}

impl<'a> Supertype<'a> {
    fn namespace(&self) -> &'a element::Namespace {
        match self {
            Supertype::Class(ns, _) | Supertype::Interface(ns, _) => ns,
        }
    }

    fn qualified_name(&self) -> String {
        let name = match self {
            Supertype::Class(_, c) => &c.name,
            Supertype::Interface(_, i) => &i.name,
        };
        format!("{}.{}", self.namespace().name, name)
    }

    fn supertypes(&self) -> Vec<&'a str> {
        match self {
            Supertype::Class(_, c) => c
                .parent
                .iter()
                .map(String::as_str)
                .chain(c.implements.iter().map(|i| i.name.as_str()))
                .collect(),
            Supertype::Interface(_, i) => match i.prerequisites.len() {
                0 => vec!["GObject.Object"],
                _ => i.prerequisites.iter().map(|p| p.name.as_str()).collect(),
            }
            .into_iter()
            .chain(i.implements.iter().map(|i| i.name.as_str()))
            .collect(),
        }
    }

    /// Signatures of methods and virtual methods (prefixed with `vfunc_`) as rendered.
    /// Members declared by overrides have an unknown signature.
    fn signatures(&self) -> Vec<(String, Option<Signature>)> {
        let ns = self.namespace();
        let (name, methods, vfuncs) = match self {
            Supertype::Class(_, c) => (&c.name, &c.methods, &c.virtual_methods),
            Supertype::Interface(_, i) => (&i.name, &i.methods, &i.virtual_methods),
        };

        let type_override = overrides::OVERRIDES
            .iter()
            .find(|o| o.namespace == ns.name && o.version == ns.version)
            .and_then(|o| o.types.iter().find(|c| c.name == *name));

        let omitted = type_override.map(|o| o.methods).unwrap_or_default();
        let declared = type_override.map(|o| o.declares).unwrap_or_default();

        let signature = |prefix: &str, m: CallableElement| {
            let (parameters, returns) = resolve_types(&m.args(prefix)).ok()?;
            let signature = Signature::new(&ns.name, &parameters, &returns);
            Some((format!("{prefix}{}", m.name()), Some(signature)))
        };

        methods
            .iter()
            .map(CallableElement::Method)
            .filter(|m| m.is_rendered() && !omitted.contains(&m.name()))
            .filter_map(|m| signature("", m))
            .chain(
                vfuncs
                    .iter()
                    .map(CallableElement::VirtualMethod)
                    .filter(|m| m.is_rendered())
                    .filter_map(|m| signature("vfunc_", m)),
            )
            .chain(declared.iter().map(|name| (name.to_string(), None)))
            .collect()
    }
}

fn find_namespace<'a>(ctx: &render::Context<'a>, name: &str) -> Option<&'a element::Namespace> {
    if name == ctx.namespace.name {
        return Some(ctx.namespace);
    }

    let version = &ctx.includes.iter().find(|i| i.name == name)?.version;

    ctx.repos
        .iter()
        .flat_map(|r| r.namespaces.iter())
        .find(|ns| ns.name == name && &ns.version == version)
}

/// Resolves a possibly qualified type name relative to `namespace`.
fn resolve<'a>(
    ctx: &render::Context<'a>,
    namespace: &'a element::Namespace,
    name: &str,
) -> Option<Supertype<'a>> {
    let (namespace, name) = match name.split_once('.') {
        Some((ns, name)) => (find_namespace(ctx, ns)?, name),
        None => (namespace, name),
    };

    namespace
        .classes
        .iter()
        .find(|c| c.name == name)
        .map(|c| Supertype::Class(namespace, c))
        .or_else(|| {
            namespace
                .interfaces
                .iter()
                .find(|i| i.name == name)
                .map(|i| Supertype::Interface(namespace, i))
        })
}

/// Every class and interface `of` inherits from, excluding itself.
fn ancestors<'a>(ctx: &render::Context<'a>, of: Supertype<'a>) -> Vec<Supertype<'a>> {
    let mut seen = HashSet::from([of.qualified_name()]);
    let mut result = Vec::new();
    let mut stack = vec![of];

    while let Some(current) = stack.pop() {
        for name in current.supertypes() {
            if let Some(t) = resolve(ctx, current.namespace(), name)
                && seen.insert(t.qualified_name())
            {
                result.push(t);
                stack.push(t);
            }
        }
    }

    result
}

/// Signatures of every member of every supertype of `owner` grouped by name.
fn inherited_signatures(
    ctx: &render::Context,
    owner: &str,
) -> HashMap<String, Vec<Option<Signature>>> {
    let mut result: HashMap<String, Vec<Option<Signature>>> = HashMap::new();

    let Some(owner) = resolve(ctx, ctx.namespace, owner) else {
        return result;
    };

    for ancestor in ancestors(ctx, owner) {
        for (name, signature) in ancestor.signatures() {
            result.entry(name).or_default().push(signature);
        }
    }

    result
}

/// Whether `sub` is not assignable to `sup` where methods are bivariant in their parameters.
fn is_conflicting(ctx: &render::Context, sub: &Signature, sup: &Signature) -> bool {
    if sub.parameters.len() > sup.parameters.len() {
        return true;
    }

    let params = sub
        .parameters
        .iter()
        .zip(sup.parameters.iter())
        .any(|(a, b)| !is_assignable(ctx, a, b) && !is_assignable(ctx, b, a));

    // anything is assignable to void
    let returns = !sup.returns.is_empty()
        && (sub.returns.len() != sup.returns.len()
            || sub
                .returns
                .iter()
                .zip(sup.returns.iter())
                .any(|(a, b)| !is_assignable(ctx, a, b)));

    params || returns
}

/// Whether `sub` is assignable to `sup`, both being qualified TypeScript types.
fn is_assignable(ctx: &render::Context, sub: &str, sup: &str) -> bool {
    if sub == sup || sup == "unknown" || sub == "never" {
        return true;
    }

    let (sub, sub_nullable) = strip_null(sub);
    let (sup, sup_nullable) = strip_null(sup);

    if sub_nullable && !sup_nullable {
        return false;
    }

    if sub == sup {
        return true;
    }

    if let Some(sub) = sub.strip_suffix("[]")
        && let Some(sup) = sup.strip_suffix("[]")
    {
        return is_assignable(ctx, sub, sup);
    }

    resolve(ctx, ctx.namespace, sub)
        .is_some_and(|t| ancestors(ctx, t).iter().any(|a| a.qualified_name() == sup))
}

fn strip_null(t: &str) -> (&str, bool) {
    match t.strip_suffix(" | null") {
        Some(t) => (t, true),
        None => (t, false),
    }
}

/// Prefixes type names in a rendered TypeScript type which are not yet qualified.
fn qualify(namespace: &str, tstype: &str) -> String {
    const BUILTINS: &[&str] = &["Record", "Uint8Array"];

    let mut result = String::with_capacity(tstype.len());
    let mut chars = tstype.char_indices().peekable();
    let mut prev: Option<char> = None;

    while let Some((start, c)) = chars.next() {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            result.push(c);
            prev = Some(c);
            continue;
        }

        let mut end = start + c.len_utf8();
        while let Some(&(i, c)) = chars.peek()
            && (c.is_ascii_alphanumeric() || c == '_')
        {
            end = i + c.len_utf8();
            chars.next();
        }

        let word = &tstype[start..end];
        let is_typename = word.starts_with(|c: char| c.is_ascii_uppercase())
            || (word.starts_with('_') && word[1..].starts_with(|c: char| c.is_ascii_digit()));
        let is_qualified = prev == Some('.') || chars.peek().is_some_and(|&(_, c)| c == '.');

        if is_typename && !is_qualified && !BUILTINS.contains(&word) {
            result.push_str(namespace);
            result.push('.');
        }

        result.push_str(word);
        prev = word.chars().last();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::qualify;

    #[test]
    fn qualifies_typenames() {
        assert_eq!(qualify("Gtk", "Widget"), "Gtk.Widget");
        assert_eq!(qualify("Gtk", "Widget | null"), "Gtk.Widget | null");
        assert_eq!(qualify("Gtk", "Gdk.Event"), "Gdk.Event");
        assert_eq!(qualify("Gtk", "Widget[]"), "Gtk.Widget[]");
        assert_eq!(qualify("Gtk", "number"), "number");
        assert_eq!(qualify("Gtk", "_2D"), "Gtk._2D");
        assert_eq!(qualify("Gtk", "Gsk._2D"), "Gsk._2D");
        assert_eq!(qualify("Gtk", "Uint8Array"), "Uint8Array");
        assert_eq!(
            qualify("Gtk", "Record<string, Widget>"),
            "Record<string, Gtk.Widget>"
        );
        assert_eq!(
            qualify("Gtk", "(GObject.GType | { $gtype: GObject.GType })"),
            "(GObject.GType | { $gtype: GObject.GType })"
        );
    }
}
