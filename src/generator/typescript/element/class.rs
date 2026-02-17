use super::super::{overrides, render};
use super::{callable, doc, gtype};
use crate::{element, generator::Event};
use stringcase::camel_case;

fn collect_signals(ctx: &render::Context, signals: &[element::Signal]) -> Vec<String> {
    signals
        .iter()
        .filter(|s| s.info.introspectable.is_none_or(|i| i))
        .filter_map(|s| {
            let detail_suffix = if s.detailed.is_some_and(|i| i) {
                "::{}"
            } else {
                ""
            };

            let args = callable::CallableArgs {
                info_elements: &s.info_elements,
                info: &s.info,
                throws: None,
                prefix: None,
                name: Some(&format!("\"{}{}\"", s.name, detail_suffix)),
                parameters: s.parameters.as_ref(),
                returns: s.return_value.as_ref(),
            };

            match callable::render(ctx, &args) {
                Ok(res) => Some(res),
                Err(err) => {
                    (ctx.event)(Event::Failed {
                        repo: None,
                        err: &format!(
                            "failed to render signal {}-{}.{}: {}",
                            ctx.namespace.name, ctx.namespace.version, s.name, err
                        ),
                    });
                    None
                }
            }
        })
        .collect()
}

fn collect_properties(
    ctx: &render::Context,
    properties: &[element::Property],
    methods: &[element::Method],
) -> Vec<minijinja::Value> {
    properties
        .iter()
        .filter(|p| p.info.introspectable.is_none_or(|i| i))
        .filter_map(|p| {
            let getter_is_nullable = p
                .getter
                .as_ref()
                .and_then(|getter| methods.iter().find(|m| m.attrs.name == *getter))
                .and_then(|m| m.return_value.as_ref())
                .is_some_and(|r| r.nullable.is_some_and(|n| n));

            let doc_args = doc::DocArgs {
                info_elements: &p.info_elements,
                info: &p.info,
                parameters: None,
                returns: None,
                throws: None,
                default_value: p.default_value.as_deref(),
            };

            match gtype::tstype(p.r#type.as_ref(), getter_is_nullable) {
                Ok(t) => Some(minijinja::context! {
                    jsdoc => doc::jsdoc_with_args(&doc_args).ok(),
                    name => &p.name,
                    type => t,
                    readable => p.readable.is_none_or(|r| r),
                    writable => p.writable.is_none_or(|w| w),
                    construct_only => p.construct_only.is_some_and(|co| co),
                }),
                Err(err) => {
                    (ctx.event)(Event::Failed {
                        repo: None,
                        err: &format!(
                            "failed to render property {}-{}.{}: {}",
                            ctx.namespace.name, ctx.namespace.version, p.name, err
                        ),
                    });
                    None
                }
            }
        })
        .collect()
}

#[derive(serde::Serialize)]
pub struct ClassContext {
    jsdoc: String,
    is_abstract: bool,
    name: String,
    name_class: String,
    parent: Option<String>,
    parent_class: Option<String>,
    extends: Vec<String>,
    signals: Vec<String>,
    properties: Vec<minijinja::Value>,
    methods: Vec<String>,
    constructors: Vec<String>,
    functions: Vec<String>,
    virtual_methods: Vec<String>,
    class_functions: Vec<String>,
}

impl render::Renderable<ClassContext> for element::Class {
    const KIND: &'static str = "class";
    const TEMPLATE: &'static str = include_str!("../templates/class.jinja");

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn env(&self, _: &render::Context, _: &ClassContext) -> minijinja::Environment<'_> {
        let mut env = minijinja::Environment::new();
        env.add_template(
            "introspection",
            include_str!("../templates/introspection.jinja"),
        )
        .unwrap();
        env.add_filter("camel_case", camel_case);
        env
    }

    fn ctx(&self, ctx: &render::Context) -> Result<ClassContext, String> {
        let extends: Vec<String> = self
            .parent
            .iter()
            .chain(self.implements.iter().map(|i| &i.name))
            .cloned()
            .collect();

        let signals = collect_signals(ctx, &self.signals);
        let properties = collect_properties(ctx, &self.properties, &self.methods);

        let overrides = overrides::OVERRIDES
            .iter()
            .find(|o| o.namespace == ctx.namespace.name && o.version == ctx.namespace.version)
            .and_then(|o| o.classes.iter().find(|c| c.name == self.name));

        let methods = callable::render_callable_elements(
            ctx,
            "",
            &self
                .methods
                .iter()
                .filter(|m| overrides.is_none_or(|o| !o.methods.iter().any(|o| o == &m.attrs.name)))
                .map(callable::CallableElement::Method)
                .collect::<Vec<_>>(),
        );

        let ctors = self
            .constructors
            .iter()
            .map(|ctor| {
                let mut c = ctor.clone();
                c.return_value = ctor.return_value.as_ref().map(|r| element::ReturnValue {
                    r#type: Some(element::AnyType::Type(element::Type {
                        name: Some(self.name.clone()),
                        c_type: None,
                        introspectable: None,
                        doc_elements: Vec::new(),
                        elements: Vec::new(),
                    })),
                    ..r.clone()
                });
                c
            })
            .collect::<Vec<_>>();

        let constructors = callable::render_callable_elements(
            ctx,
            "",
            &ctors
                .iter()
                .map(callable::CallableElement::Constructor)
                .collect::<Vec<_>>(),
        );

        let functions = callable::render_callable_elements(
            ctx,
            "",
            &self
                .functions
                .iter()
                .map(callable::CallableElement::Function)
                .collect::<Vec<_>>(),
        );

        let virtual_methods = callable::render_callable_elements(
            ctx,
            "vfunc_",
            &self
                .virtual_methods
                .iter()
                .map(callable::CallableElement::VirtualMethod)
                .collect::<Vec<_>>(),
        );

        let constructor_record = ctx.namespace.records.iter().find(|rec| {
            rec.glib_is_gtype_struct_for
                .as_ref()
                .is_some_and(|name| name == &self.name)
        });

        let class_functions = constructor_record
            .map(|record| {
                callable::render_callable_elements(
                    ctx,
                    "",
                    &record
                        .methods
                        .iter()
                        .map(callable::CallableElement::Method)
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default();

        let name_class = constructor_record
            .map(|rec| rec.name.clone())
            // TODO: check for possible name collision
            .unwrap_or_else(|| format!("{}Class", &self.name));

        let parent_class = self.parent.as_ref().map(|parent| {
            ctx.namespace
                .records
                .iter()
                .find(|rec| {
                    rec.glib_is_gtype_struct_for
                        .as_ref()
                        .is_some_and(|name| name == parent)
                })
                .map(|rec| rec.name.clone())
                // TODO: check for possible name collision
                .unwrap_or_else(|| format!("{}Class", parent))
        });

        Ok(ClassContext {
            jsdoc: doc::jsdoc(&self.info_elements, &self.info).unwrap(),
            is_abstract: self.r#abstract.is_some_and(|i| i),
            name: self.name.clone(),
            name_class,
            parent: self.parent.clone(),
            parent_class,
            extends,
            signals,
            properties,
            methods,
            constructors,
            functions,
            virtual_methods,
            class_functions,
        })
    }
}

impl render::Renderable<ClassContext> for element::Interface {
    const KIND: &'static str = "interface";
    const TEMPLATE: &'static str = include_str!("../templates/interface.jinja");

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn env(&self, _: &render::Context, _: &ClassContext) -> minijinja::Environment<'_> {
        let mut env = minijinja::Environment::new();
        env.add_template(
            "introspection",
            include_str!("../templates/introspection.jinja"),
        )
        .unwrap();
        env.add_filter("camel_case", camel_case);
        env
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<ClassContext, String> {
        let prereqs: Vec<&str> = match self.prerequisites.len() {
            0 => vec!["GObject.Object"],
            _ => self.prerequisites.iter().map(|p| p.name.as_ref()).collect(),
        };

        let impls: Vec<&str> = self.implements.iter().map(|i| i.name.as_ref()).collect();

        let extends: Vec<String> = [prereqs, impls]
            .concat()
            .into_iter()
            .map(String::from)
            .collect();

        let signals = collect_signals(ctx, &self.signals);
        let properties = collect_properties(ctx, &self.properties, &self.methods);

        let overrides = overrides::OVERRIDES
            .iter()
            .find(|o| o.namespace == ctx.namespace.name && o.version == ctx.namespace.version)
            .and_then(|o| o.classes.iter().find(|c| c.name == self.name));

        let methods = callable::render_callable_elements(
            ctx,
            "",
            &self
                .methods
                .iter()
                .filter(|m| overrides.is_none_or(|o| !o.methods.iter().any(|o| o == &m.attrs.name)))
                .map(callable::CallableElement::Method)
                .collect::<Vec<_>>(),
        );

        let constructors = callable::render_callable_elements(
            ctx,
            "",
            &self
                .constructors
                .iter()
                .map(callable::CallableElement::Constructor)
                .collect::<Vec<_>>(),
        );

        let functions = callable::render_callable_elements(
            ctx,
            "",
            &self
                .functions
                .iter()
                .map(callable::CallableElement::Function)
                .collect::<Vec<_>>(),
        );

        let virtual_methods = callable::render_callable_elements(
            ctx,
            "vfunc_",
            &self
                .virtual_methods
                .iter()
                .map(callable::CallableElement::VirtualMethod)
                .collect::<Vec<_>>(),
        );

        let constructor_record = ctx.namespace.records.iter().find(|rec| {
            rec.glib_is_gtype_struct_for
                .as_ref()
                .is_some_and(|name| name == &self.name)
        });

        let class_functions = constructor_record
            .map(|iface| {
                callable::render_callable_elements(
                    ctx,
                    "",
                    &iface
                        .methods
                        .iter()
                        .map(callable::CallableElement::Method)
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default();

        let name_class = constructor_record
            .map(|rec| rec.name.clone())
            // TODO: check for possible name collision
            .unwrap_or_else(|| format!("{}Iface", &self.name));

        Ok(ClassContext {
            is_abstract: false,
            parent: None,
            parent_class: None,
            jsdoc: doc::jsdoc(&self.info_elements, &self.info).unwrap(),
            name: self.name.clone(),
            name_class,
            extends,
            signals,
            properties,
            methods,
            constructors,
            functions,
            virtual_methods,
            class_functions,
        })
    }
}
