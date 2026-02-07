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

pub struct DocArgs<'a> {
    pub info_elements: &'a [element::InfoElement],
    pub info: &'a element::InfoAttrs,
    pub parameters: Option<&'a element::Parameters>,
    pub returns: Option<&'a element::ReturnValue>,
    pub throws: Option<bool>,
    pub overrides: Option<bool>,
    pub default_value: Option<&'a str>,
}

#[derive(serde::Serialize)]
struct Doc<'a> {
    text_lines: Vec<&'a str>,
    throws: bool,
    overrides: bool,
    since: Option<&'a str>,
    deprecated: bool,
    deprecated_since: Option<&'a str>,
    deprecated_text: Option<&'a str>,
    experimental: bool,
    default_value: Option<&'a str>,
    parameters: Vec<DocParameter<'a>>,
    returns: Option<&'a str>,
}

#[derive(serde::Serialize)]
struct DocParameter<'a> {
    name: &'a str,
    text: &'a str,
}

pub fn jsdoc(args: &[element::InfoElement], info: &element::InfoAttrs) -> Result<String, String> {
    todo!()
}

pub fn jsdoc_with_args(args: &DocArgs) -> Result<String, String> {
    todo!()
}
