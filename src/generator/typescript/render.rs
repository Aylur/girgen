use super::overrides;
use crate::{element, generator::Event};
use rayon::prelude::*;
use rayon::scope;

pub struct Context<'a> {
    pub namespace: &'a element::Namespace,
    pub event: fn(Event),
}

fn escape_member(name: &str) -> String {
    match name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        true => format!("\"{name}\""),
        false => name.to_owned(),
    }
}

fn escape_toplevel(name: &str) -> String {
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return format!("_{name}");
    }

    match name {
        "void" | "enum" | "function" | "false" | "true" | "break" | "boolean" => format!("_{name}"),
        _ => name.to_owned(),
    }
}

pub trait Renderable<T: serde::Serialize> {
    const KIND: &'static str;
    const TEMPLATE: &'static str;

    fn name(&self, _: &Context) -> &str;
    fn introspectable(&self, _: &Context) -> bool;
    fn ctx(&self, _: &Context) -> Result<T, String>;

    fn env(&self, _: &Context, _: &T) -> minijinja::Environment<'_> {
        minijinja::Environment::new()
    }

    fn render(&self, g_ctx: &Context) -> Result<String, String> {
        let name = self.name(g_ctx);

        let ctx = match self.ctx(g_ctx) {
            Ok(ctx) => ctx,
            Err(err) => {
                return Err(format!(
                    "rendering {} {}-{}.{}: {}",
                    Self::KIND,
                    g_ctx.namespace.name,
                    g_ctx.namespace.version,
                    name,
                    err
                ));
            }
        };

        let mut env = self.env(g_ctx, &ctx);

        env.add_filter("escape_member", escape_member);
        env.add_filter("escape_toplevel", escape_toplevel);

        env.render_str(Self::TEMPLATE, &ctx).map_err(|err| {
            format!(
                "rendering {} {}-{}.{}: {:?}",
                Self::KIND,
                g_ctx.namespace.name,
                g_ctx.namespace.version,
                self.name(g_ctx),
                err
            )
        })
    }
}

fn render<R: serde::Serialize, T: Renderable<R> + Sync>(items: &[T], ctx: &Context) -> Vec<String> {
    let overrides = overrides::OVERRIDES
        .iter()
        .find(|o| o.namespace == ctx.namespace.name && o.version == ctx.namespace.version);

    items
        .par_iter()
        .filter_map(|elem| {
            if !elem.introspectable(ctx) {
                return None;
            }

            if overrides.is_some_and(|o| o.toplevel.iter().any(|n| *n == elem.name(ctx))) {
                return None;
            }

            match elem.render(ctx) {
                Ok(elem) => Some(elem),
                Err(err) => {
                    (ctx.event)(Event::Failed {
                        repo: None,
                        err: err.as_str(),
                    });
                    None
                }
            }
        })
        .collect()
}

#[derive(serde::Serialize)]
struct RenderedNamespace<'a> {
    name: &'a str,
    version: &'a str,
    extra_content: &'a str,
    aliases: Vec<String>,
    classes: Vec<String>,
    interfaces: Vec<String>,
    records: Vec<String>,
    enums: Vec<String>,
    functions: Vec<String>,
    unions: Vec<String>,
    bitfields: Vec<String>,
    callbacks: Vec<String>,
    constants: Vec<String>,
}

fn render_namespace<'a>(ctx: Context<'a>) -> RenderedNamespace<'a> {
    let overrides = overrides::OVERRIDES
        .iter()
        .find(|o| o.namespace == ctx.namespace.name && o.version == ctx.namespace.version);

    let mut aliases = None;
    let mut classes = None;
    let mut interfaces = None;
    let mut records = None;
    let mut enums_ = None;
    let mut functions = None;
    let mut unions = None;
    let mut bitfields = None;
    let mut callbacks = None;
    let mut constants = None;

    scope(|s| {
        s.spawn(|_| aliases = Some(render(&ctx.namespace.aliases, &ctx)));
        s.spawn(|_| classes = Some(render(&ctx.namespace.classes, &ctx)));
        s.spawn(|_| interfaces = Some(render(&ctx.namespace.interfaces, &ctx)));
        s.spawn(|_| records = Some(render(&ctx.namespace.records, &ctx)));
        s.spawn(|_| enums_ = Some(render(&ctx.namespace.enums, &ctx)));
        s.spawn(|_| functions = Some(render(&ctx.namespace.functions, &ctx)));
        s.spawn(|_| unions = Some(render(&ctx.namespace.unions, &ctx)));
        s.spawn(|_| bitfields = Some(render(&ctx.namespace.bitfields, &ctx)));
        s.spawn(|_| callbacks = Some(render(&ctx.namespace.callbacks, &ctx)));
        s.spawn(|_| constants = Some(render(&ctx.namespace.constants, &ctx)));
    });

    RenderedNamespace {
        name: &ctx.namespace.name,
        version: &ctx.namespace.version,
        extra_content: overrides.and_then(|o| o.content).unwrap_or_default(),
        aliases: aliases.unwrap(),
        classes: classes.unwrap(),
        interfaces: interfaces.unwrap(),
        records: records.unwrap(),
        enums: enums_.unwrap(),
        functions: functions.unwrap(),
        unions: unions.unwrap(),
        bitfields: bitfields.unwrap(),
        callbacks: callbacks.unwrap(),
        constants: constants.unwrap(),
    }
}

#[derive(serde::Serialize)]
struct Import<'a> {
    name: &'a str,
    version: &'a str,
}

impl element::Repository {
    fn find_imports(&self, repos: &[&element::Repository]) -> Vec<element::Include> {
        let mut includes = self.find_includes(repos);

        let namespace = self
            .namespaces
            .first()
            .map(|ns| format!("{}-{}", ns.name, ns.version))
            .unwrap();

        let has_gobject = includes
            .iter()
            .any(|inc| inc.name == "GObject" && inc.version == "2.0");

        if namespace != "GObject-2.0" && !has_gobject {
            includes.push(element::Include {
                name: String::from("GObject"),
                version: String::from("2.0"),
            });
        }

        let has_glib = includes
            .iter()
            .any(|inc| inc.name == "GLib" && inc.version == "2.0");

        if namespace != "GLib-2.0" && !has_glib {
            includes.push(element::Include {
                name: String::from("GLib"),
                version: String::from("2.0"),
            });
        }

        includes
    }

    pub fn generate_dts(
        &self,
        repos: &[&element::Repository],
        event: fn(Event),
    ) -> Result<String, String> {
        let namespaces = self
            .namespaces
            .par_iter()
            .map(|namespace| render_namespace(Context { namespace, event }))
            .collect::<Vec<_>>();

        let includes = self.find_imports(repos);
        let imports: Vec<Import> = includes
            .iter()
            .map(|inc| Import {
                name: &inc.name,
                version: &inc.version,
            })
            .collect();

        let res = minijinja::Environment::new().render_str(
            include_str!("templates/repository.jinja"),
            minijinja::context! { imports, namespaces },
        );

        match res {
            Ok(res) => Ok(res),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
