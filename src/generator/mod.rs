mod cache;
pub mod typescript;

pub use cache::{cache, hash, lookup_cache};

use crate::element;
use std::{io, path};

pub enum Event<'a> {
    Parsed {
        file_path: &'a path::Path,
    },
    ParseFailed {
        file_path: &'a path::Path,
        err: &'a str,
    },
    Ignored {
        file_path: &'a path::Path,
        cause: &'a str,
    },
    Warning {
        warning: &'a str,
    },
    Failed {
        repo: Option<&'a str>,
        err: &'a str,
    },
    Generated {
        repo: &'a str,
        out_path: &'a str,
    },
    CacheHit {
        repo: &'a str,
        out_path: &'a str,
    },
}

pub enum Error {
    Empty,
    FsError(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::FsError(value)
    }
}

pub type Generator<T> =
    fn(opts: T, girs: &[Gir], outdir: &str, event: fn(Event)) -> Result<(), Error>;

pub struct Gir<'a> {
    pub name: &'a str,
    pub contents: String,
    pub repo: element::Repository,
}
