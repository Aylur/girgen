mod alias;
mod annotation;
mod bitfield;
mod boxed;
mod callable;
mod callback;
mod cinclude;
mod class;
mod constant;
mod constructor;
mod doc;
mod doc_elements;
mod r#enum;
mod field;
mod function;
mod info;
mod interface;
mod member;
mod method;
mod namespace;
mod package;
mod property;
mod record;
mod repository;
mod signal;
mod r#type;
mod union;

pub use super::error::ParseError;
pub use alias::*;
pub use annotation::*;
pub use bitfield::*;
pub use boxed::*;
pub use callable::*;
pub use callback::*;
pub use cinclude::*;
pub use class::*;
pub use constant::*;
pub use constructor::*;
pub use doc::*;
pub use doc_elements::*;
pub use r#enum::*;
pub use field::*;
pub use function::*;
pub use info::*;
pub use interface::*;
pub use member::*;
pub use method::*;
pub use namespace::*;
pub use package::*;
pub use property::*;
pub use record::*;
pub use repository::*;
pub use signal::*;
pub use r#type::*;
pub use union::*;

pub struct Attrs(pub std::collections::HashMap<String, String>);

pub enum Required<T> {
    Ok(T),
    Missing,
}

impl Attrs {
    fn get_string(&self, key: &str) -> Result<String, ParseError> {
        let res = self
            .0
            .get(key)
            .ok_or(ParseError::MissingAttribute(String::from(key)))?;

        Ok(res.to_owned())
    }

    fn get_boolean(&self, key: &str) -> Result<bool, ParseError> {
        let value = self.get_string(key)?;
        match value.as_ref() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParseError::MalformedGir("invalid boolean value")),
        }
    }

    fn get_int(&self, key: &str) -> Result<i32, ParseError> {
        let value = self.get_string(key)?;
        value
            .parse()
            .map_err(|_| ParseError::MalformedGir("invalid int"))
    }
}

pub trait Element {
    const KIND: &'static str;

    fn new(attrs: &Attrs) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn end(&mut self, element: AnyElement) -> Result<(), ParseError> {
        Err(ParseError::UnexpectedElement(format!(
            "{} is expected to be empty, but got {}",
            Self::KIND,
            element.kind(),
        )))
    }

    fn text(&mut self, str: &str) -> Result<(), ParseError> {
        Err(ParseError::UnexpectedElement(str.to_owned()))
    }
}

pub enum AnyElement {
    Repository(Repository),
    Namespace(Namespace),
    Alias(Alias),
    Array(Array),
    Attribute(Attribute),
    Bitfield(Bitfield),
    Boxed(Boxed),
    CInclude(CInclude),
    Callback(Callback),
    Class(Class),
    Constant(Constant),
    Constructor(Constructor),
    Doc(Doc),
    DocDeprecated(DocDeprecated),
    DocFormat(DocFormat),
    DocSection(DocSection),
    DocStability(DocStability),
    DocVersion(DocVersion),
    Enumeration(Enumeration),
    Field(Field),
    Function(Function),
    FunctionInline(FunctionInline),
    FunctionMacro(FunctionMacro),
    Implements(Implements),
    Include(Include),
    InstanceParameter(InstanceParameter),
    Interface(Interface),
    Member(Member),
    Method(Method),
    MethodInline(MethodInline),
    Package(Package),
    Parameter(Parameter),
    Parameters(Parameters),
    Prerequisite(Prerequisite),
    Property(Property),
    Record(Record),
    ReturnValue(ReturnValue),
    Signal(Signal),
    SourcePosition(SourcePosition),
    Type(Type),
    Union(Union),
    VarArgs(VarArgs),
    VirtualMethod(VirtualMethod),
}

// TODO: benchmark vs Box<dyn Element> to reduce these
impl AnyElement {
    pub fn kind(&self) -> &'static str {
        match self {
            AnyElement::Repository(_) => Repository::KIND,
            AnyElement::Namespace(_) => Namespace::KIND,
            AnyElement::Alias(_) => Alias::KIND,
            AnyElement::Array(_) => Array::KIND,
            AnyElement::Attribute(_) => Attribute::KIND,
            AnyElement::Bitfield(_) => Bitfield::KIND,
            AnyElement::Boxed(_) => Boxed::KIND,
            AnyElement::CInclude(_) => CInclude::KIND,
            AnyElement::Callback(_) => Callback::KIND,
            AnyElement::Class(_) => Class::KIND,
            AnyElement::Constant(_) => Constant::KIND,
            AnyElement::Constructor(_) => Constructor::KIND,
            AnyElement::Doc(_) => Doc::KIND,
            AnyElement::DocDeprecated(_) => DocDeprecated::KIND,
            AnyElement::DocFormat(_) => DocFormat::KIND,
            AnyElement::DocSection(_) => DocSection::KIND,
            AnyElement::DocStability(_) => DocStability::KIND,
            AnyElement::DocVersion(_) => DocVersion::KIND,
            AnyElement::Enumeration(_) => Enumeration::KIND,
            AnyElement::Field(_) => Field::KIND,
            AnyElement::Function(_) => Function::KIND,
            AnyElement::FunctionInline(_) => FunctionInline::KIND,
            AnyElement::FunctionMacro(_) => FunctionMacro::KIND,
            AnyElement::Implements(_) => Implements::KIND,
            AnyElement::Include(_) => Include::KIND,
            AnyElement::InstanceParameter(_) => InstanceParameter::KIND,
            AnyElement::Interface(_) => Interface::KIND,
            AnyElement::Member(_) => Member::KIND,
            AnyElement::Method(_) => Method::KIND,
            AnyElement::MethodInline(_) => MethodInline::KIND,
            AnyElement::Package(_) => Package::KIND,
            AnyElement::Parameter(_) => Parameter::KIND,
            AnyElement::Parameters(_) => Parameters::KIND,
            AnyElement::Prerequisite(_) => Prerequisite::KIND,
            AnyElement::Property(_) => Property::KIND,
            AnyElement::Record(_) => Record::KIND,
            AnyElement::ReturnValue(_) => ReturnValue::KIND,
            AnyElement::Signal(_) => Signal::KIND,
            AnyElement::SourcePosition(_) => SourcePosition::KIND,
            AnyElement::Type(_) => Type::KIND,
            AnyElement::Union(_) => Union::KIND,
            AnyElement::VarArgs(_) => VarArgs::KIND,
            AnyElement::VirtualMethod(_) => VirtualMethod::KIND,
        }
    }

    pub fn new(name: &[u8], attrs: &Attrs) -> Result<Self, ParseError> {
        let ele = match name {
            b"namespace" => Self::Namespace(Namespace::new(attrs)?),
            b"attribute" => Self::Attribute(Attribute::new(attrs)?),
            b"c:include" => Self::CInclude(CInclude::new(attrs)?),
            b"doc:format" => Self::DocFormat(DocFormat::new(attrs)?),
            b"include" => Self::Include(Include::new(attrs)?),
            b"package" => Self::Package(Package::new(attrs)?),
            b"alias" => Self::Alias(Alias::new(attrs)?),
            b"interface" => Self::Interface(Interface::new(attrs)?),
            b"class" => Self::Class(Class::new(attrs)?),
            b"glib:boxed" => Self::Boxed(Boxed::new(attrs)?),
            b"record" => Self::Record(Record::new(attrs)?),
            b"doc" => Self::Doc(Doc::new(attrs)?),
            b"doc-deprecated" => Self::DocDeprecated(DocDeprecated::new(attrs)?),
            b"doc-stability" => Self::DocStability(DocStability::new(attrs)?),
            b"doc-version" => Self::DocVersion(DocVersion::new(attrs)?),
            b"source-position" => Self::SourcePosition(SourcePosition::new(attrs)?),
            b"constant" => Self::Constant(Constant::new(attrs)?),
            b"property" => Self::Property(Property::new(attrs)?),
            b"glib:signal" => Self::Signal(Signal::new(attrs)?),
            b"field" => Self::Field(Field::new(attrs)?),
            b"callback" => Self::Callback(Callback::new(attrs)?),
            b"implements" => Self::Implements(Implements::new(attrs)?),
            b"prerequisite" => Self::Prerequisite(Prerequisite::new(attrs)?),
            b"type" => Self::Type(Type::new(attrs)?),
            b"array" => Self::Array(Array::new(attrs)?),
            b"constructor" => Self::Constructor(Constructor::new(attrs)?),
            b"varargs" => Self::VarArgs(VarArgs::new(attrs)?),
            b"parameters" => Self::Parameters(Parameters::new(attrs)?),
            b"parameter" => Self::Parameter(Parameter::new(attrs)?),
            b"instance-parameter" => Self::InstanceParameter(InstanceParameter::new(attrs)?),
            b"return-value" => Self::ReturnValue(ReturnValue::new(attrs)?),
            b"function" => Self::Function(Function::new(attrs)?),
            b"function-inline" => Self::FunctionInline(FunctionInline::new(attrs)?),
            b"function-macro" => Self::FunctionMacro(FunctionMacro::new(attrs)?),
            b"method" => Self::Method(Method::new(attrs)?),
            b"method-inline" => Self::MethodInline(MethodInline::new(attrs)?),
            b"virtual-method" => Self::VirtualMethod(VirtualMethod::new(attrs)?),
            b"union" => Self::Union(Union::new(attrs)?),
            b"bitfield" => Self::Bitfield(Bitfield::new(attrs)?),
            b"enumeration" => Self::Enumeration(Enumeration::new(attrs)?),
            b"member" => Self::Member(Member::new(attrs)?),
            b"docsection" => Self::DocSection(DocSection::new(attrs)?),
            tag => {
                return Err(ParseError::UnhandledXmlTag(
                    str::from_utf8(tag).unwrap().to_owned(),
                ));
            }
        };
        Ok(ele)
    }

    pub fn end(&mut self, e: AnyElement) -> Result<(), ParseError> {
        match self {
            AnyElement::Repository(i) => i.end(e),
            AnyElement::Namespace(i) => i.end(e),
            AnyElement::Alias(i) => i.end(e),
            AnyElement::Array(i) => i.end(e),
            AnyElement::Attribute(i) => i.end(e),
            AnyElement::Bitfield(i) => i.end(e),
            AnyElement::Boxed(i) => i.end(e),
            AnyElement::CInclude(i) => i.end(e),
            AnyElement::Callback(i) => i.end(e),
            AnyElement::Class(i) => i.end(e),
            AnyElement::Constant(i) => i.end(e),
            AnyElement::Constructor(i) => i.end(e),
            AnyElement::Doc(i) => i.end(e),
            AnyElement::DocDeprecated(i) => i.end(e),
            AnyElement::DocFormat(i) => i.end(e),
            AnyElement::DocSection(i) => i.end(e),
            AnyElement::DocStability(i) => i.end(e),
            AnyElement::DocVersion(i) => i.end(e),
            AnyElement::Enumeration(i) => i.end(e),
            AnyElement::Field(i) => i.end(e),
            AnyElement::Function(i) => i.end(e),
            AnyElement::FunctionInline(i) => i.end(e),
            AnyElement::FunctionMacro(i) => i.end(e),
            AnyElement::Implements(i) => i.end(e),
            AnyElement::Include(i) => i.end(e),
            AnyElement::InstanceParameter(i) => i.end(e),
            AnyElement::Interface(i) => i.end(e),
            AnyElement::Member(i) => i.end(e),
            AnyElement::Method(i) => i.end(e),
            AnyElement::MethodInline(i) => i.end(e),
            AnyElement::Package(i) => i.end(e),
            AnyElement::Parameter(i) => i.end(e),
            AnyElement::Parameters(i) => i.end(e),
            AnyElement::Prerequisite(i) => i.end(e),
            AnyElement::Property(i) => i.end(e),
            AnyElement::Record(i) => i.end(e),
            AnyElement::ReturnValue(i) => i.end(e),
            AnyElement::Signal(i) => i.end(e),
            AnyElement::SourcePosition(i) => i.end(e),
            AnyElement::Type(i) => i.end(e),
            AnyElement::Union(i) => i.end(e),
            AnyElement::VarArgs(i) => i.end(e),
            AnyElement::VirtualMethod(i) => i.end(e),
        }
    }

    pub fn text(&mut self, str: &str) -> Result<(), ParseError> {
        match self {
            AnyElement::Repository(i) => i.text(str),
            AnyElement::Namespace(i) => i.text(str),
            AnyElement::Alias(i) => i.text(str),
            AnyElement::Array(i) => i.text(str),
            AnyElement::Attribute(i) => i.text(str),
            AnyElement::Bitfield(i) => i.text(str),
            AnyElement::Boxed(i) => i.text(str),
            AnyElement::CInclude(i) => i.text(str),
            AnyElement::Callback(i) => i.text(str),
            AnyElement::Class(i) => i.text(str),
            AnyElement::Constant(i) => i.text(str),
            AnyElement::Constructor(i) => i.text(str),
            AnyElement::Doc(i) => i.text(str),
            AnyElement::DocDeprecated(i) => i.text(str),
            AnyElement::DocFormat(i) => i.text(str),
            AnyElement::DocSection(i) => i.text(str),
            AnyElement::DocStability(i) => i.text(str),
            AnyElement::DocVersion(i) => i.text(str),
            AnyElement::Enumeration(i) => i.text(str),
            AnyElement::Field(i) => i.text(str),
            AnyElement::Function(i) => i.text(str),
            AnyElement::FunctionInline(i) => i.text(str),
            AnyElement::FunctionMacro(i) => i.text(str),
            AnyElement::Implements(i) => i.text(str),
            AnyElement::Include(i) => i.text(str),
            AnyElement::InstanceParameter(i) => i.text(str),
            AnyElement::Interface(i) => i.text(str),
            AnyElement::Member(i) => i.text(str),
            AnyElement::Method(i) => i.text(str),
            AnyElement::MethodInline(i) => i.text(str),
            AnyElement::Package(i) => i.text(str),
            AnyElement::Parameter(i) => i.text(str),
            AnyElement::Parameters(i) => i.text(str),
            AnyElement::Prerequisite(i) => i.text(str),
            AnyElement::Property(i) => i.text(str),
            AnyElement::Record(i) => i.text(str),
            AnyElement::ReturnValue(i) => i.text(str),
            AnyElement::Signal(i) => i.text(str),
            AnyElement::SourcePosition(i) => i.text(str),
            AnyElement::Type(i) => i.text(str),
            AnyElement::Union(i) => i.text(str),
            AnyElement::VarArgs(i) => i.text(str),
            AnyElement::VirtualMethod(i) => i.text(str),
        }
    }
}
