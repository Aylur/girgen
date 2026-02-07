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
                overrides: false,
                prefix: None,
                name: Some(&format!("\"{}{}\"", s.name, detail_suffix)),
                parameters: s.parameters.as_ref(),
                returns: s.return_value.as_ref(),
            };

            match callable::render(&args) {
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
                overrides: None,
                default_value: p.default_value.as_deref(),
            };

            match gtype::tstype(p.r#type.as_ref(), getter_is_nullable) {
                Ok(t) => Some(minijinja::context! {
                    jsdoc => doc::jsdoc_with_args(&doc_args).ok(),
                    name => &p.name,
                    type => t,
                    readable => p.readable,
                    writable => p.writable,
                    construct_only => p.construct_only,
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

impl render::Renderable for element::Class {
    const KIND: &'static str = "class";
    const TEMPLATE: &'static str = include_str!("../templates/class.jinja");

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i)
    }

    fn env(&self, _: &render::Context) -> minijinja::Environment<'_> {
        let mut env = minijinja::Environment::new();
        env.add_template(
            "introspection",
            include_str!("../templates/introspection.jinja"),
        )
        .unwrap();
        env.add_filter("camel_case", camel_case);
        env
    }

    fn ctx(&self, ctx: &render::Context) -> Result<minijinja::Value, String> {
        let extends: Vec<&String> = self
            .parent
            .iter()
            .chain(self.implements.iter().map(|i| &i.name))
            .collect();

        let signals = collect_signals(ctx, &self.signals);
        let properties = collect_properties(ctx, &self.properties, &self.methods);

        let overrides = overrides::OVERRIDES
            .iter()
            .find(|o| o.namespace == ctx.namespace.name && o.version == ctx.namespace.version)
            .and_then(|o| o.classes.iter().find(|c| c.name == &self.name));

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

        // TODO: consider overriding return value with self.name
        // example: currently `Adw.AboutDialog.new` returns `Adw.Dialog`
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

        let class_functions = constructor_record.map(|record| {
            callable::render_callable_elements(
                ctx,
                "",
                &record
                    .methods
                    .iter()
                    .map(callable::CallableElement::Method)
                    .collect::<Vec<_>>(),
            )
        });

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

        Ok(minijinja::context! {
            jsdoc => doc::jsdoc(&self.info_elements, &self.info).unwrap(),
            is_abstarct => self.r#abstract.is_some_and(|i| i),
            name => &self.name,
            name_class,
            parent => self.parent,
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

impl render::Renderable for element::Interface {
    const KIND: &'static str = "interface";
    const TEMPLATE: &'static str = include_str!("../templates/interface.jinja");

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn env(&self, _: &render::Context) -> minijinja::Environment<'_> {
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

    fn ctx(&self, ctx: &render::Context) -> Result<minijinja::Value, String> {
        let extends: Vec<&String> = self
            .prerequisites
            .iter()
            .map(|p| &p.name)
            .chain(self.implements.iter().map(|i| &i.name))
            .collect();

        let signals = collect_signals(ctx, &self.signals);
        let properties = collect_properties(ctx, &self.properties, &self.methods);

        let overrides = overrides::OVERRIDES
            .iter()
            .find(|o| o.namespace == ctx.namespace.name && o.version == ctx.namespace.version)
            .and_then(|o| o.classes.iter().find(|c| c.name == &self.name));

        let methods = callable::render_callable_elements(
            &ctx,
            "",
            &self
                .methods
                .iter()
                .filter(|m| overrides.is_none_or(|o| !o.methods.iter().any(|o| o == &m.attrs.name)))
                .map(callable::CallableElement::Method)
                .collect::<Vec<_>>(),
        );

        let constructors = callable::render_callable_elements(
            &ctx,
            "",
            &self
                .constructors
                .iter()
                .map(callable::CallableElement::Constructor)
                .collect::<Vec<_>>(),
        );

        let functions = callable::render_callable_elements(
            &ctx,
            "",
            &self
                .functions
                .iter()
                .map(callable::CallableElement::Function)
                .collect::<Vec<_>>(),
        );

        let virtual_methods = callable::render_callable_elements(
            &ctx,
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

        let class_functions = constructor_record.map(|iface| {
            callable::render_callable_elements(
                ctx,
                "",
                &iface
                    .methods
                    .iter()
                    .map(callable::CallableElement::Method)
                    .collect::<Vec<_>>(),
            )
        });

        let name_iface = constructor_record
            .map(|rec| rec.name.clone())
            // TODO: check for possible name collision
            .unwrap_or_else(|| format!("{}Iface", &self.name));

        Ok(minijinja::context! {
            jsdoc => doc::jsdoc(&self.info_elements, &self.info).unwrap(),
            name => &self.name,
            name_iface,
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
