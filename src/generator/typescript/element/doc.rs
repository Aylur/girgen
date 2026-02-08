use super::gtype;
use crate::element;
use regex::Regex;
use std::sync::LazyLock;

static TEMPLATE: &str = include_str!("../templates/doc.jinja");
static DOC_AT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:^|\s)@(\w+)").unwrap());

// TODO: format gi-docgen links
// e.g `[method@NS.Class.method]` -> `{@link NS.Class.prototype.method}`
fn fmt(text: &str) -> String {
    DOC_AT_RE
        .replace_all(text, r" `$1`")
        .into_owned()
        .replace('\n', " ")
}

fn get_doc_text(info: &[element::DocElement]) -> String {
    info.iter()
        .filter_map(|info| match info {
            element::DocElement::Doc(doc) => Some(doc.text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub struct DocArgs<'a> {
    pub info_elements: &'a [element::InfoElement],
    pub info: &'a element::InfoAttrs,
    pub parameters: Option<&'a element::Parameters>,
    pub returns: Option<&'a element::ReturnValue>,
    pub throws: Option<bool>,
    pub overrides: bool,
    pub default_value: Option<&'a str>,
}

#[derive(serde::Serialize)]
struct DocParameter<'a> {
    name: &'a str,
    text: String,
}

pub fn jsdoc(
    info_elements: &[element::InfoElement],
    info: &element::InfoAttrs,
) -> Result<String, String> {
    let args = DocArgs {
        info_elements,
        info,
        parameters: None,
        returns: None,
        throws: None,
        overrides: false,
        default_value: None,
    };
    jsdoc_with_args(&args)
}

pub fn jsdoc_with_args(args: &DocArgs) -> Result<String, String> {
    let mut doc: String = String::new();
    let mut doc_deprecated: Option<&str> = None;
    let mut doc_stability: Option<&str> = None;

    for info in args.info_elements {
        match info {
            element::InfoElement::Annotation(_) => {}
            element::InfoElement::DocElement(doc_element) => match doc_element {
                element::DocElement::DocDeprecated(d) => doc_deprecated = Some(&d.text),
                element::DocElement::DocStability(d) => doc_stability = Some(&d.text),
                element::DocElement::Doc(d) => doc.push_str(&d.text),
                _ => {}
            },
        }
    }

    let text_lines: Vec<&str> = doc.lines().collect();
    let deprecated_text: Option<String> = doc_deprecated.map(fmt);
    let parameters = gtype::filter_parameters(args.parameters, args.returns);

    let in_parameters: Vec<DocParameter> = parameters
        .iter()
        .filter(|p| matches!(p.direction.as_deref(), None | Some("in")))
        .map(|p| DocParameter {
            name: p.name.as_deref().unwrap_or("arg"),
            text: get_doc_text(&p.doc_elements),
        })
        .collect();

    let out_parameters: Vec<String> = args
        .returns
        .as_ref()
        .map(|ret| get_doc_text(&ret.doc_elements))
        .into_iter()
        .chain(
            parameters
                .iter()
                .filter(|p| matches!(p.direction.as_deref(), Some("out" | "inout")))
                .map(|p| get_doc_text(&p.doc_elements)),
        )
        .map(|s| fmt(&s))
        .collect();

    let experimental = doc_stability.is_some_and(|s| s == "Unstable");

    let env = minijinja::Environment::new();

    let ctx = minijinja::context! {
        text_lines => text_lines,
        throws => args.throws,
        overrides => args.overrides,
        since => args.info.version,
        deprecated => args.info.deprecated,
        deprecated_since => args.info.deprecated_version,
        deprecated_text => deprecated_text,
        experimental => experimental,
        default_value => args.default_value,
        parameters => in_parameters,
        returns => out_parameters.join(", "),
    };

    env.render_str(TEMPLATE, ctx)
        .map_err(|err| format!("failed to render jsdoc: error: {:?}", err))
}
