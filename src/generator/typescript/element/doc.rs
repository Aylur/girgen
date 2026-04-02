use super::gtype;
use crate::element;
use regex::{Captures, Regex};
use std::sync::LazyLock;
use stringcase::camel_case;

static TEMPLATE: &str = include_str!("../templates/doc.jinja");
static DOC_AT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:^|\s)@(\w+)").unwrap());
static GI_DOCDGEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(\w+)@([^\]]+)\]").unwrap());

fn format_gi_docgen(caps: &Captures<'_>) -> String {
    let kind = &caps[1];
    let target = &caps[2];

    match kind {
        "property" => {
            if let Some((prefix, prop)) = target.rsplit_once(':') {
                format!("{{@link {}.{}}}", prefix, camel_case(prop))
            } else {
                format!("{{@link {}}}", target)
            }
        }
        "signal" => {
            if let Some((prefix, signal)) = target.split_once("::") {
                format!(r#"{{@link {}.SignalSignatures["{}"]}}"#, prefix, signal)
            } else {
                format!("{{@link {}}}", target)
            }
        }
        "vfunc" => {
            if let Some((prefix, func)) = target.rsplit_once('.') {
                format!("{{@link {}.vfunc_{}}}", prefix, func)
            } else {
                format!("{{@link {}}}", target)
            }
        }
        _ => format!("{{@link {}}}", target),
    }
}

fn escape_doc(text: &str) -> String {
    let text = GI_DOCDGEN_RE.replace_all(text, format_gi_docgen);
    let text = DOC_AT_RE.replace_all(&text, r" `$1`");
    text.replace('\n', " ")
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

    let text_lines: Vec<String> = doc.lines().map(escape_doc).collect();
    let deprecated_text: Option<String> = doc_deprecated.map(escape_doc);
    let parameters = gtype::filter_parameters(args.parameters, args.returns);

    let in_parameters: Vec<DocParameter> = parameters
        .iter()
        .filter(|p| matches!(p.direction.as_deref(), None | Some("in")))
        .map(|p| DocParameter {
            name: p.name.as_deref().unwrap_or("arg"),
            text: escape_doc(&get_doc_text(&p.doc_elements)),
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
        .map(|s| escape_doc(&s))
        .collect();

    let experimental = doc_stability.is_some_and(|s| s == "Unstable");

    let env = minijinja::Environment::new();

    let ctx = minijinja::context! {
        text_lines => text_lines,
        throws => args.throws,
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
