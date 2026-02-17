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
    ($ns:expr, $self:expr, $name:expr, $ctx:expr, $ctor:expr) => {{
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
            constructor: $ctor,
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
        // TODO: generate a constructor for non opaque records
        ctx!(ctx.namespace.name, self, self.name.clone(), ctx, None)
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
        ctx!(ctx.namespace.name, self, name, ctx, None)
    }
}
