use super::super::render;
use super::{callable, doc, gtype};
use crate::element;
use stringcase::snake_case;

const TEMPLATE: &str = include_str!("../templates/record.jinja");

#[derive(serde::Serialize)]
pub struct FieldContext {
    jsdoc: String,
    name: String,
    r#type: String,
    readable: bool,
    writable: bool,
}

macro_rules! ctx {
    ($ns:expr, $self:expr, $name:expr, $ctx:expr) => {{
        let jsdoc = doc::jsdoc(&$self.info_elements, &$self.info)?;

        let fields: Vec<FieldContext> = $self
            .fields
            .iter()
            .filter(|f| f.info.introspectable.is_none_or(|i| i) && f.private.is_none_or(|p| !p))
            .filter_map(|f| {
                let gtype = match gtype::tstype(f.r#type.as_ref(), false) {
                    Ok(ok) => ok,
                    Err(_) => return None,
                };

                let jsdoc = doc::jsdoc(&f.info_elements, &f.info).unwrap();

                Some(FieldContext {
                    jsdoc,
                    name: snake_case(&f.name),
                    r#type: gtype,
                    readable: f.readable.unwrap_or(false),
                    writable: f.writable.unwrap_or(false),
                })
            })
            .collect();

        let methods = callable::render_callable_elements(
            $ctx,
            "",
            &$self
                .methods
                .iter()
                .map(callable::CallableElement::Method)
                .collect::<Vec<_>>(),
        );

        let constructors = callable::render_callable_elements(
            $ctx,
            "",
            &$self
                .constructors
                .iter()
                .map(callable::CallableElement::Constructor)
                .collect::<Vec<_>>(),
        );

        let functions = callable::render_callable_elements(
            $ctx,
            "",
            &$self
                .functions
                .iter()
                .map(callable::CallableElement::Function)
                .collect::<Vec<_>>(),
        );

        Ok(RecordContext {
            namespace: $ns.clone(),
            jsdoc,
            name: $name,
            bag_constructor: false,
            constructor: None,
            fields,
            methods,
            constructors,
            functions,
        })
    }};
}

#[derive(serde::Serialize)]
pub struct RecordContext {
    namespace: String,
    jsdoc: String,
    name: String,
    bag_constructor: bool,
    constructor: Option<String>,
    fields: Vec<FieldContext>,
    methods: Vec<String>,
    constructors: Vec<String>,
    functions: Vec<String>,
}

impl render::Renderable<RecordContext> for element::Record {
    const KIND: &'static str = "record";
    const TEMPLATE: &'static str = TEMPLATE;

    fn name(&self, _: &render::Context) -> &str {
        &self.name
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        // gtype structs are rendered in classes/interfaces
        self.glib_is_gtype_struct_for.is_none() && self.info.introspectable.is_none_or(|i| i)
    }

    fn ctx(&self, ctx: &render::Context) -> Result<RecordContext, String> {
        let is_opaque = self.opaque.is_some_and(|o| o);
        let mut first_ctor: Option<&element::Constructor> = None;
        let mut zero_arg_ctor: Option<&element::Constructor> = None;
        let mut default_ctor: Option<&element::Constructor> = None;

        for ctor in self.constructors.iter() {
            if first_ctor.is_none() {
                first_ctor = Some(ctor);
            }

            let zero_args = ctor
                .parameters
                .as_ref()
                .is_none_or(|p| p.parameters.is_empty());

            if zero_args && zero_arg_ctor.is_none() {
                zero_arg_ctor = Some(ctor);
            }

            if ctor.attrs.name == "new" && default_ctor.is_none() {
                default_ctor = Some(ctor);
            }
        }

        if default_ctor.is_none()
            && let Some(ctor) = zero_arg_ctor
        {
            default_ctor = Some(ctor);
        }

        if default_ctor.is_none()
            && let Some(ctor) = first_ctor
        {
            default_ctor = Some(ctor);
        }

        let bag_constructor = !is_opaque || zero_arg_ctor.is_some();

        match ctx!(ctx.namespace.name, self, self.name.clone(), ctx) {
            Err(err) => Err(err),
            Ok(rec) => Ok(RecordContext {
                bag_constructor,
                constructor: default_ctor.and_then(|ctor| {
                    if bag_constructor {
                        return None;
                    }

                    let render = callable::render(
                        ctx,
                        &callable::CallableArgs {
                            info_elements: &ctor.info_elements,
                            info: &ctor.attrs.info,
                            throws: None,
                            prefix: Some("new"),
                            name: Some(" "),
                            parameters: ctor.parameters.as_ref(),
                            returns: ctor.return_value.as_ref(),
                        },
                    );

                    render.ok()
                }),
                ..rec
            }),
        }
    }
}

impl render::Renderable<RecordContext> for element::Union {
    const KIND: &'static str = "union";
    const TEMPLATE: &'static str = TEMPLATE;

    fn name(&self, _: &render::Context) -> &str {
        match &self.name {
            Some(name) => name,
            None => "",
        }
    }

    fn introspectable(&self, _: &render::Context) -> bool {
        self.info.introspectable.is_none_or(|i| i) && self.name.is_some()
    }

    fn ctx(&self, ctx: &render::Context) -> Result<RecordContext, String> {
        let name = self.name.clone().unwrap_or_default();
        ctx!(ctx.namespace.name, self, name, ctx)
    }
}
