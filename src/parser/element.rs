mod alias;
mod annotation;
mod bitfield;
mod boxed;
mod callable;
mod callback;
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
        Err(ParseError::UnexpectedElement(Self::KIND, element.kind()))
    }

    fn text(&mut self, _str: &str) -> Result<(), ParseError> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum AnyElement {
    Invalid,
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

impl AnyElement {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Invalid => "invalid",
            Self::Repository(_) => Repository::KIND,
            Self::Namespace(_) => Namespace::KIND,
            Self::Alias(_) => Alias::KIND,
            Self::Array(_) => Array::KIND,
            Self::Attribute(_) => Attribute::KIND,
            Self::Bitfield(_) => Bitfield::KIND,
            Self::Boxed(_) => Boxed::KIND,
            Self::CInclude(_) => CInclude::KIND,
            Self::Callback(_) => Callback::KIND,
            Self::Class(_) => Class::KIND,
            Self::Constant(_) => Constant::KIND,
            Self::Constructor(_) => Constructor::KIND,
            Self::Doc(_) => Doc::KIND,
            Self::DocDeprecated(_) => DocDeprecated::KIND,
            Self::DocFormat(_) => DocFormat::KIND,
            Self::DocSection(_) => DocSection::KIND,
            Self::DocStability(_) => DocStability::KIND,
            Self::DocVersion(_) => DocVersion::KIND,
            Self::Enumeration(_) => Enumeration::KIND,
            Self::Field(_) => Field::KIND,
            Self::Function(_) => Function::KIND,
            Self::FunctionInline(_) => FunctionInline::KIND,
            Self::FunctionMacro(_) => FunctionMacro::KIND,
            Self::Implements(_) => Implements::KIND,
            Self::Include(_) => Include::KIND,
            Self::InstanceParameter(_) => InstanceParameter::KIND,
            Self::Interface(_) => Interface::KIND,
            Self::Member(_) => Member::KIND,
            Self::Method(_) => Method::KIND,
            Self::MethodInline(_) => MethodInline::KIND,
            Self::Package(_) => Package::KIND,
            Self::Parameter(_) => Parameter::KIND,
            Self::Parameters(_) => Parameters::KIND,
            Self::Prerequisite(_) => Prerequisite::KIND,
            Self::Property(_) => Property::KIND,
            Self::Record(_) => Record::KIND,
            Self::ReturnValue(_) => ReturnValue::KIND,
            Self::Signal(_) => Signal::KIND,
            Self::SourcePosition(_) => SourcePosition::KIND,
            Self::Type(_) => Type::KIND,
            Self::Union(_) => Union::KIND,
            Self::VarArgs(_) => VarArgs::KIND,
            Self::VirtualMethod(_) => VirtualMethod::KIND,
        }
    }

    pub fn new(name: &str, attrs: &Attrs) -> Result<Self, ParseError> {
        let ele = match name {
            "namespace" => Self::Namespace(Namespace::new(attrs)?),
            "attribute" => Self::Attribute(Attribute::new(attrs)?),
            "c:include" => Self::CInclude(CInclude::new(attrs)?),
            "doc:format" => Self::DocFormat(DocFormat::new(attrs)?),
            "include" => Self::Include(Include::new(attrs)?),
            "package" => Self::Package(Package::new(attrs)?),
            "alias" => Self::Alias(Alias::new(attrs)?),
            "interface" => Self::Interface(Interface::new(attrs)?),
            "class" => Self::Class(Class::new(attrs)?),
            "glib:boxed" => Self::Boxed(Boxed::new(attrs)?),
            "record" => Self::Record(Record::new(attrs)?),
            "doc" => Self::Doc(Doc::new(attrs)?),
            "doc-deprecated" => Self::DocDeprecated(DocDeprecated::new(attrs)?),
            "doc-stability" => Self::DocStability(DocStability::new(attrs)?),
            "doc-version" => Self::DocVersion(DocVersion::new(attrs)?),
            "source-position" => Self::SourcePosition(SourcePosition::new(attrs)?),
            "constant" => Self::Constant(Constant::new(attrs)?),
            "property" => Self::Property(Property::new(attrs)?),
            "glib:signal" => Self::Signal(Signal::new(attrs)?),
            "field" => Self::Field(Field::new(attrs)?),
            "callback" => Self::Callback(Callback::new(attrs)?),
            "implements" => Self::Implements(Implements::new(attrs)?),
            "prerequisite" => Self::Prerequisite(Prerequisite::new(attrs)?),
            "type" => Self::Type(Type::new(attrs)?),
            "array" => Self::Array(Array::new(attrs)?),
            "constructor" => Self::Constructor(Constructor::new(attrs)?),
            "varargs" => Self::VarArgs(VarArgs::new(attrs)?),
            "parameters" => Self::Parameters(Parameters::new(attrs)?),
            "parameter" => Self::Parameter(Parameter::new(attrs)?),
            "instance-parameter" => Self::InstanceParameter(InstanceParameter::new(attrs)?),
            "return-value" => Self::ReturnValue(ReturnValue::new(attrs)?),
            "function" => Self::Function(Function::new(attrs)?),
            "function-inline" => Self::FunctionInline(FunctionInline::new(attrs)?),
            "function-macro" => Self::FunctionMacro(FunctionMacro::new(attrs)?),
            "method" => Self::Method(Method::new(attrs)?),
            "method-inline" => Self::MethodInline(MethodInline::new(attrs)?),
            "virtual-method" => Self::VirtualMethod(VirtualMethod::new(attrs)?),
            "union" => Self::Union(Union::new(attrs)?),
            "bitfield" => Self::Bitfield(Bitfield::new(attrs)?),
            "enumeration" => Self::Enumeration(Enumeration::new(attrs)?),
            "member" => Self::Member(Member::new(attrs)?),
            "docsection" => Self::DocSection(DocSection::new(attrs)?),
            tag => return Err(ParseError::InvalidGirElement(tag.to_owned())),
        };
        Ok(ele)
    }

    pub fn end(&mut self, e: AnyElement) -> Result<(), ParseError> {
        if matches!(e, Self::Invalid) {
            return Ok(());
        }

        match self {
            Self::Invalid => Ok(()),
            Self::Repository(i) => i.end(e),
            Self::Namespace(i) => i.end(e),
            Self::Alias(i) => i.end(e),
            Self::Array(i) => i.end(e),
            Self::Attribute(i) => i.end(e),
            Self::Bitfield(i) => i.end(e),
            Self::Boxed(i) => i.end(e),
            Self::CInclude(i) => i.end(e),
            Self::Callback(i) => i.end(e),
            Self::Class(i) => i.end(e),
            Self::Constant(i) => i.end(e),
            Self::Constructor(i) => i.end(e),
            Self::Doc(i) => i.end(e),
            Self::DocDeprecated(i) => i.end(e),
            Self::DocFormat(i) => i.end(e),
            Self::DocSection(i) => i.end(e),
            Self::DocStability(i) => i.end(e),
            Self::DocVersion(i) => i.end(e),
            Self::Enumeration(i) => i.end(e),
            Self::Field(i) => i.end(e),
            Self::Function(i) => i.end(e),
            Self::FunctionInline(i) => i.end(e),
            Self::FunctionMacro(i) => i.end(e),
            Self::Implements(i) => i.end(e),
            Self::Include(i) => i.end(e),
            Self::InstanceParameter(i) => i.end(e),
            Self::Interface(i) => i.end(e),
            Self::Member(i) => i.end(e),
            Self::Method(i) => i.end(e),
            Self::MethodInline(i) => i.end(e),
            Self::Package(i) => i.end(e),
            Self::Parameter(i) => i.end(e),
            Self::Parameters(i) => i.end(e),
            Self::Prerequisite(i) => i.end(e),
            Self::Property(i) => i.end(e),
            Self::Record(i) => i.end(e),
            Self::ReturnValue(i) => i.end(e),
            Self::Signal(i) => i.end(e),
            Self::SourcePosition(i) => i.end(e),
            Self::Type(i) => i.end(e),
            Self::Union(i) => i.end(e),
            Self::VarArgs(i) => i.end(e),
            Self::VirtualMethod(i) => i.end(e),
        }
    }

    pub fn text(&mut self, str: &str) -> Result<(), ParseError> {
        match self {
            Self::Invalid => Ok(()),
            Self::Repository(i) => i.text(str),
            Self::Namespace(i) => i.text(str),
            Self::Alias(i) => i.text(str),
            Self::Array(i) => i.text(str),
            Self::Attribute(i) => i.text(str),
            Self::Bitfield(i) => i.text(str),
            Self::Boxed(i) => i.text(str),
            Self::CInclude(i) => i.text(str),
            Self::Callback(i) => i.text(str),
            Self::Class(i) => i.text(str),
            Self::Constant(i) => i.text(str),
            Self::Constructor(i) => i.text(str),
            Self::Doc(i) => i.text(str),
            Self::DocDeprecated(i) => i.text(str),
            Self::DocFormat(i) => i.text(str),
            Self::DocSection(i) => i.text(str),
            Self::DocStability(i) => i.text(str),
            Self::DocVersion(i) => i.text(str),
            Self::Enumeration(i) => i.text(str),
            Self::Field(i) => i.text(str),
            Self::Function(i) => i.text(str),
            Self::FunctionInline(i) => i.text(str),
            Self::FunctionMacro(i) => i.text(str),
            Self::Implements(i) => i.text(str),
            Self::Include(i) => i.text(str),
            Self::InstanceParameter(i) => i.text(str),
            Self::Interface(i) => i.text(str),
            Self::Member(i) => i.text(str),
            Self::Method(i) => i.text(str),
            Self::MethodInline(i) => i.text(str),
            Self::Package(i) => i.text(str),
            Self::Parameter(i) => i.text(str),
            Self::Parameters(i) => i.text(str),
            Self::Prerequisite(i) => i.text(str),
            Self::Property(i) => i.text(str),
            Self::Record(i) => i.text(str),
            Self::ReturnValue(i) => i.text(str),
            Self::Signal(i) => i.text(str),
            Self::SourcePosition(i) => i.text(str),
            Self::Type(i) => i.text(str),
            Self::Union(i) => i.text(str),
            Self::VarArgs(i) => i.text(str),
            Self::VirtualMethod(i) => i.text(str),
        }
    }
}
