use crate::element::{AnyType, Array, Parameter, Parameters, ReturnValue};

pub fn filter_parameters<'a>(
    params: Option<&'a Parameters>,
    returns: Option<&'a ReturnValue>,
) -> Vec<&'a Parameter> {
    let params = match params {
        Some(p) => &p.parameters,
        None => return Vec::new(),
    };

    let mut remove = vec![false; params.len() + 1];
    let mut result = Vec::new();

    if let Some(ret) = returns
        && let Some(AnyType::Array(t)) = &ret.r#type
        && let Some(length) = t.length
        && let Ok(i) = usize::try_from(length)
    {
        remove[i] = true;
    }

    for param in params.iter() {
        if let Some(destroy) = param.destroy
            && let Ok(i) = usize::try_from(destroy)
        {
            remove[i] = true;
        }
        if let Some(closure) = param.closure
            && let Ok(i) = usize::try_from(closure)
        {
            remove[i] = true;
        }
        if let Some(AnyType::Array(t)) = &param.r#type
            && let Some(length) = t.length
            && let Ok(i) = usize::try_from(length)
        {
            remove[i] = true;
        }
    }

    for (i, param) in params.iter().enumerate() {
        if !remove[i] {
            result.push(param);
        }
    }

    result
}

enum TypeError<'a> {
    MissingName,
    Failed(&'a str),
    UnhandledGeneric(&'a str),
    UnhandledElement(&'a Array),
    MissingContent(&'a Array),
}

fn resolve_typename<'a>(typename: &'a str) -> Result<String, TypeError<'a>> {
    if typename.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return Ok(format!("_{typename}"));
    }

    let mut words = typename.split(".");
    if let Some(namespace) = words.next()
        && let Some(name) = words.next()
        && name.chars().next().is_some_and(|c| c.is_ascii_digit())
    {
        return Ok(format!("{namespace}._{name}"));
    }

    match typename {
        "GType" => Ok("GObject.GType".to_string()),
        "gunichar" | "filename" | "utf8" => Ok("string".to_string()),
        "void" | "none" => Ok("void".to_string()),
        "uint" | "int" | "uint8" | "int8" | "uint16" | "int16" | "uint32" | "int32" | "int64"
        | "uint64" | "double" | "long" | "long double" | "float" | "gshort" | "guint32"
        | "guint16" | "gint16" | "gint8" | "gint32" | "gushort" | "gfloat" | "gchar" | "guint"
        | "glong" | "gulong" | "gint" | "guint8" | "guint64" | "gint64" | "gdouble" | "gssize"
        | "gsize" | "time_t" | "uid_t" | "pid_t" | "ulong" => Ok("number".to_string()),
        "gboolean" => Ok("boolean".to_string()),
        "object" => Ok("object".to_string()),
        "gpointer" | "gintptr" | "guintptr" => Ok("never".to_string()),
        t if t.contains(".") => Ok(t.to_string()),
        t if t.chars().next().is_some_and(|c| c.is_uppercase()) => Ok(t.to_string()),
        // TODO:
        t if t.ends_with("_t") || t.starts_with("_") => Ok("never".to_string()),
        // TODO: some libraries use lowercase names
        t => Err(TypeError::Failed(t)),
    }
}

fn resolve_anytype<'a>(input: &'a AnyType) -> Result<String, TypeError<'a>> {
    match input {
        AnyType::Type(t) => {
            let name = t.name.as_ref().ok_or(TypeError::MissingName)?;

            match name.as_str() {
                "GLib.List" | "GLib.SList" => {
                    if let Some(g) = t.elements.first() {
                        Ok(format!("{}[]", resolve_anytype(g)?))
                    } else {
                        Ok(name.clone())
                    }
                }
                "GLib.HashTable" => {
                    let mut i = t.elements.iter();
                    if let Some(k) = i.next()
                        && let Some(v) = i.next()
                    {
                        Ok(format!(
                            "Record<{}, {}>",
                            resolve_anytype(k)?,
                            resolve_anytype(v)?
                        ))
                    } else {
                        Ok(name.clone())
                    }
                }
                _ if !t.elements.is_empty() => Err(TypeError::UnhandledGeneric(name)),
                _ => resolve_typename(name),
            }
        }
        AnyType::Array(arr) => {
            let ele = arr.elements.first().ok_or(TypeError::MissingContent(arr))?;

            if arr.elements.len() > 1 {
                return Err(TypeError::UnhandledElement(arr));
            }

            if let AnyType::Type(item) = ele
                && let Some(name) = item.name.as_ref()
            {
                match name.as_str() {
                    "gint8" | "guint8" => return Ok("Uint8Array".to_string()),
                    "gunichar" => return Ok("string".to_string()),
                    _ => (),
                }
            }

            Ok(format!("{}[]", resolve_anytype(ele)?))
        }
    }
}

pub fn tstype(anytype: Option<&AnyType>, nullable: bool) -> Result<String, String> {
    let r#type = anytype.ok_or("missing type".to_string())?;

    let tstype = match resolve_anytype(r#type) {
        Ok(ok) => Ok(ok),
        Err(err) => match err {
            TypeError::MissingName => Err("missing type name".to_string()),
            TypeError::Failed(name) => Err(format!("failed to resolve type '{name}'")),
            TypeError::UnhandledGeneric(name) => {
                if name.starts_with("GLib") {
                    eprintln!("unhandled generic type: {}", name);
                }
                Err(format!("unhandled generic type: {}", name))
            }
            TypeError::UnhandledElement(array) => {
                Err(format!("unhandled array elements {:?}", array))
            }
            TypeError::MissingContent(array) => Err(format!("missing array element {:?}", array)),
        },
    }?;

    if nullable {
        Ok(format!("{tstype} | null"))
    } else {
        Ok(tstype)
    }
}
